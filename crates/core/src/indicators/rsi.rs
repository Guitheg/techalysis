/*
    BSD 3-Clause License

    Copyright (c) 2025, Guillaume GOBIN (Guitheg)

    Redistribution and use in source and binary forms, with or without modification,
    are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation and/or
    other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors
    may be used to endorse or promote products derived from this software without
    specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
    THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

/*
    List of contributors:
    - Guitheg: Initial implementation
*/

/*
    Inspired by TA-LIB RSI implementation
*/

//! Relative Strength Index (RSI) implementation

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// RSI calculation result
/// ---
/// This struct holds the result and the state ([`RsiState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated RSI values.
/// - `state`: A [`RsiState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct RsiResult {
    /// The calculated RSI values.
    pub values: Vec<Float>,
    /// A [`RsiState`], which can be used to calculate
    /// the next values incrementally.
    pub state: RsiState,
}

/// RSI calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `rsi`: The last calculated RSI value.
///
/// **State values**
/// - `prev_value`: The previous input value used for the RSI calculation.
/// - `avg_gain`: The average gain calculated from the input data.
/// - `avg_loss`: The average loss calculated from the input data.
///
/// **Parameters**
/// - `period`: The period used for the RSI calculation.
#[derive(Debug, Clone, Copy)]
pub struct RsiState {
    // Outputs
    /// The last calculated RSI value.
    pub rsi: Float,

    // State values
    /// The previous input value used for the RSI calculation.
    pub prev_value: Float,
    /// The average gain calculated from the input data.
    pub avg_gain: Float,
    /// The average loss calculated from the input data.
    pub avg_loss: Float,

    // Parameters
    /// The period used for the RSI calculation.
    pub period: usize,
}

impl State<Float> for RsiState {
    /// Update the [`RsiState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the RSI state.
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        if self.period <= 1 {
            return Err(TechalibError::BadParam(
                "RSI period must be greater than 1".to_string(),
            ));
        }

        if !sample.is_finite() {
            return Err(TechalibError::DataNonFinite(
                format!("sample = {sample:?}",),
            ));
        }
        if !self.prev_value.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "prev_value = {:?}",
                self.prev_value
            )));
        }
        if !self.avg_gain.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.avg_gain = {:?}",
                self.avg_gain
            )));
        }
        if !self.avg_loss.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.avg_loss = {:?}",
                self.avg_loss
            )));
        }

        let (rsi, avg_gain, avg_loss) = rsi_next_unchecked(
            sample - self.prev_value,
            self.avg_gain,
            self.avg_loss,
            1.0 / self.period as Float,
        );
        if !rsi.is_finite() {
            return Err(TechalibError::Overflow(0, rsi));
        }
        self.rsi = rsi;
        self.prev_value = sample;
        self.avg_gain = avg_gain;
        self.avg_loss = avg_loss;
        Ok(())
    }
}

/// Lookback period for RSI calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    TechalibError::check_period(period)?;
    Ok(period)
}

/// Calculation of the RSI function
/// ---
/// It returns a [`RsiResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the RSI calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`RsiResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn rsi(data: &[Float], period: usize) -> Result<RsiResult, TechalibError> {
    let size: usize = data.len();
    let mut output = vec![0.0; size];
    let rsi_state = rsi_into(data, period, output.as_mut_slice())?;
    Ok(RsiResult {
        values: output,
        state: rsi_state,
    })
}

/// Calculation of the RSI function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`RsiState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the RSI calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the RSI values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`RsiState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn rsi_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<RsiState, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;
    TechalibError::check_period(period)?;
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    if len <= period {
        return Err(TechalibError::InsufficientData);
    }

    let mut avg_gain: Float = 0.0;
    let mut avg_loss: Float = 0.0;
    output[0] = Float::NAN;
    TechalibError::check_finite_at(0, data)?;
    for i in 1..=period {
        TechalibError::check_finite_at(i, data)?;
        let delta = data[i] - data[i - 1];
        if delta > 0.0 {
            avg_gain += delta;
        } else {
            avg_loss -= delta;
        }
        output[i] = Float::NAN;
    }
    avg_gain *= inv_period;
    avg_loss *= inv_period;
    output[period] = calculate_rsi(avg_gain, avg_loss);
    TechalibError::check_overflow_at(period, output)?;

    for i in (period + 1)..len {
        TechalibError::check_finite_at(i, data)?;
        (output[i], avg_gain, avg_loss) =
            rsi_next_unchecked(data[i] - data[i - 1], avg_gain, avg_loss, inv_period);
        TechalibError::check_overflow_at(i, output)?;
    }
    Ok(RsiState {
        rsi: output[len - 1],
        prev_value: data[len - 1],
        avg_gain,
        avg_loss,
        period,
    })
}

#[inline(always)]
fn rsi_next_unchecked(
    delta: Float,
    prev_avg_gain: Float,
    prev_avg_loss: Float,
    inv_period: Float,
) -> (Float, Float, Float) {
    let one_minus_k = 1.0 - inv_period;
    let (avg_gain, avg_loss) = if delta > 0.0 {
        (
            prev_avg_gain * one_minus_k + delta * inv_period,
            prev_avg_loss * one_minus_k,
        )
    } else if delta < 0.0 {
        (
            prev_avg_gain * one_minus_k,
            prev_avg_loss * one_minus_k - delta * inv_period,
        )
    } else {
        (prev_avg_gain * one_minus_k, prev_avg_loss * one_minus_k)
    };

    (calculate_rsi(avg_gain, avg_loss), avg_gain, avg_loss)
}

#[inline(always)]
fn calculate_rsi(avg_gain: Float, avg_loss: Float) -> Float {
    if avg_loss == 0.0 {
        if avg_gain == 0.0 {
            return 50.0;
        }
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}
