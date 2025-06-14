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
    Inspired by TA-LIB SMA implementation
*/

//! Simple Moving Average (SMA) implementation

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;
use std::collections::VecDeque;

/// SMA calculation result
/// ---
/// This struct holds the result and the state ([`SmaState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated SMA values.
/// - `state`: A [`SmaState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct SmaResult {
    /// The calculated SMA values.
    pub values: Vec<Float>,
    /// A [`SmaState`], which can be used to calculate
    /// the next values incrementally.
    pub state: SmaState,
}

/// SMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `sma`: The last calculated Simple Moving Average (SMA) value.
///
/// **State values**
/// - `last_window`: A deque containing the last `period` values used for
///   the SMA calculation.
///
/// **Parameters**
/// - `period`: The period used for the SMA calculation, which determines
///   how many values are averaged to compute the SMA.
#[derive(Debug, Clone)]
pub struct SmaState {
    // Outputs
    /// The last calculated Simple Moving Average (SMA) value.
    pub sma: Float,

    // State values
    /// A deque containing the last `period` values used for
    /// the SMA calculation.
    pub last_window: VecDeque<Float>,

    // Parameters
    /// The period used for the SMA calculation, which determines
    /// how many values are averaged to compute the SMA.
    pub period: usize,
}

impl State<Float> for SmaState {
    /// Update the [`SmaState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the SMA state
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_period(self.period)?;
        TechalibError::check_finite(self.sma, "sma")?;
        TechalibError::check_finite(sample, "sample")?;

        if self.last_window.len() != self.period {
            return Err(TechalibError::BadParam(format!(
                "SMA state last_window length ({}) does not match period ({})",
                self.last_window.len(),
                self.period
            )));
        }

        for (idx, &value) in self.last_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalibError::DataNonFinite(format!(
                    "window[{idx}] = {value:?}"
                )));
            }
        }

        let mut window = self.last_window.clone();

        let old_value = window.pop_front().ok_or(TechalibError::InsufficientData)?;
        window.push_back(sample);

        let sma = sma_next_unchecked(sample, old_value, self.sma, 1.0 / (self.period as Float));
        TechalibError::check_overflow(sma)?;
        self.sma = sma;
        self.last_window = window;

        Ok(())
    }
}

/// Lookback period for SMA calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n` first values that will be return will be `NaN`
/// and the next values will be the values.
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    TechalibError::check_period(period)?;
    Ok(period - 1)
}

/// Calculation of the SMA function
/// ---
/// It returns a [`SmaResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
///
/// Returns
/// ---
/// A `Result` containing a [`SmaResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn sma(data: &[Float], period: usize) -> Result<SmaResult, TechalibError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let sma_state = sma_into(data, period, &mut output)?;
    Ok(SmaResult {
        values: output,
        state: sma_state,
    })
}

/// Calculation of the SMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`SmaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the SMA calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated SMA values
///   will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`SmaState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn sma_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<SmaState, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;
    let len = data.len();
    let inv_period = 1.0 / (period as Float);
    let lookback = lookback_from_period(period)?;
    if period > len {
        return Err(TechalibError::InsufficientData);
    }

    output[lookback] = init_sma_unchecked(data, period, inv_period, output)?;
    TechalibError::check_overflow_at(lookback, output)?;

    for idx in period..len {
        TechalibError::check_finite_at(idx, data)?;
        output[idx] =
            sma_next_unchecked(data[idx], data[idx - period], output[idx - 1], inv_period);
        TechalibError::check_overflow_at(idx, output)?;
    }
    Ok(SmaState {
        sma: output[len - 1],
        period,
        last_window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

#[inline(always)]
pub(crate) fn sma_next_unchecked(
    new_value: Float,
    old_value: Float,
    prev_sma: Float,
    inv_period: Float,
) -> Float {
    prev_sma + (new_value - old_value) * inv_period
}

#[inline(always)]
pub(crate) fn init_sma_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    output: &mut [Float],
) -> Result<Float, TechalibError> {
    let mut sum: Float = 0.0;
    for idx in 0..period {
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        } else {
            sum += value;
        }
        output[idx] = Float::NAN;
    }
    Ok(sum * inv_period)
}
