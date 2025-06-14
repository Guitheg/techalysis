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
    Inspired by TA-LIB EMA implementation
*/

//! Exponential Moving Average (EMA) implementation

use crate::errors::TechalibError;
use crate::indicators::sma::init_sma_unchecked;
use crate::traits::State;
use crate::types::Float;

const DEFAULT_SMOOTHING: Float = 2.0;

/// EMA calculation result
/// ---
/// This struct holds the result and the state ([`EmaState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated EMA values.
/// - `state`: A [`EmaState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct EmaResult {
    /// The calculated EMA values.
    pub values: Vec<Float>,
    /// A [`EmaState`], which can be used to calculate the next values
    /// incrementally.
    pub state: EmaState,
}

/// EMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `ema`: The last calculated Exponential Moving Average (EMA) value.
///
/// **Parameters**
/// - `period`: The period used for the EMA calculation.
/// - `alpha`: The alpha factor used in the EMA calculation.
///   Traditionally, it is calculated as `smoothing / (period + 1)`.
#[derive(Debug, Clone, Copy)]
pub struct EmaState {
    // Outputs values
    /// The last calculated Exponential Moving Average (EMA) value.
    pub ema: Float,

    // Parameters
    /// The period used for the EMA calculation.
    pub period: usize,
    /// The alpha factor used in the EMA calculation
    /// Traditionally, it is calculated as `smoothing / (period + 1)`.
    pub alpha: Float,
}

impl State<Float> for EmaState {
    /// Update the [`EmaState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the EMA state.
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_period(self.period)?;
        TechalibError::check_finite(self.ema, "ema")?;
        TechalibError::check_finite(sample, "sample")?;
        TechalibError::check_finite(self.alpha, "alpha")?;
        let ema = ema_next_unchecked(sample, self.ema, self.alpha);
        TechalibError::check_overflow(ema)?;
        self.ema = ema;
        Ok(())
    }
}

/// Lookback period for EMA calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    TechalibError::check_period(period)?;
    Ok(period - 1)
}

/// Calculation of the EMA function
/// ---
/// It returns a [`EmaResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the EMA calculation.
/// - `alpha`: An optional alpha value for the EMA calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`EmaResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn ema(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<EmaResult, TechalibError> {
    let mut output = vec![0.0; data.len()];
    let ema_state = ema_into(data, period, alpha, &mut output)?;
    Ok(EmaResult {
        values: output,
        state: ema_state,
    })
}

/// Calculation of the EMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`EmaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the EMA calculation.
/// - `alpha`: An optional alpha value for the EMA calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated EMA values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`EmaState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn ema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<EmaState, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;

    let len = data.len();
    let inv_period = 1.0 / period as Float;
    let lookback = lookback_from_period(period)?;

    if len < period {
        return Err(TechalibError::InsufficientData);
    }

    let alpha = get_alpha_value(alpha, period)?;

    output[lookback] = init_sma_unchecked(data, period, inv_period, output)?;
    TechalibError::check_overflow_at(lookback, output)?; // Weird line

    for idx in lookback + 1..len {
        TechalibError::check_finite_at(idx, data)?;
        output[idx] = ema_next_unchecked(data[idx], output[idx - 1], alpha);
        TechalibError::check_overflow_at(idx, output)?;
    }

    Ok(EmaState {
        ema: output[len - 1],
        period,
        alpha,
    })
}

/// Converts a period to an alpha value for EMA calculation.
/// According to the formula:
/// alpha = smoothing / (period + 1)
///
/// Input Arguments
/// ---
/// - `period`: The period for which to calculate the alpha value.
/// - `smoothing`: Optional smoothing factor, defaults to 2.0 if not provided.
///
/// Returns
/// ---
/// A `Result` containing the calculated alpha value as `Float`, or a
/// [`TechalibError`] if the period is invalid or if the smoothing factor is invalid.
pub fn period_to_alpha(period: usize, smoothing: Option<Float>) -> Result<Float, TechalibError> {
    if period == 0 {
        return Err(TechalibError::BadParam(
            "Period must be greater than 0".to_string(),
        ));
    }

    let smoothing = match smoothing {
        Some(s) => {
            if s <= 0.0 {
                return Err(TechalibError::BadParam(
                    "Smoothing must be greater than 0".to_string(),
                ));
            }
            s
        }
        None => DEFAULT_SMOOTHING,
    };

    Ok(smoothing / (period as Float + 1.0))
}

#[inline(always)]
pub(crate) fn ema_next_unchecked(new_value: Float, prev_ema: Float, alpha: Float) -> Float {
    new_value * alpha + prev_ema * (1.0 - alpha)
}

#[inline(always)]
pub(crate) fn get_alpha_value(alpha: Option<Float>, period: usize) -> Result<Float, TechalibError> {
    match alpha {
        Some(a) => Ok(a),
        None => period_to_alpha(period, None),
    }
}
