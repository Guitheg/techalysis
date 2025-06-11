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

use crate::errors::TechalysisError;
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
    fn update(&mut self, sample: Float) -> Result<(), TechalysisError> {
        if self.period <= 1 {
            return Err(TechalysisError::BadParam(
                "Period must be greater than 1".to_string(),
            ));
        }

        if !sample.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "sample = {sample:?}",
            )));
        }

        if !self.ema.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema = {:?}",
                self.ema
            )));
        }

        if !self.alpha.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "alpha = {:?}",
                self.alpha
            )));
        }

        let ema = ema_next_unchecked(sample, self.ema, self.alpha);
        if !ema.is_finite() {
            return Err(TechalysisError::Overflow(0, ema));
        }
        self.ema = ema;
        Ok(())
    }
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
/// or a [`TechalysisError`] error if the calculation fails.
pub fn ema(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<EmaResult, TechalysisError> {
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
/// or a [`TechalysisError`] error if the calculation fails.
pub fn ema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<EmaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    if period == 0 || len < period {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "EMA period must be greater than 1".to_string(),
        ));
    }

    let alpha = get_alpha_value(alpha, period)?;

    output[period - 1] = init_sma_unchecked(data, period, inv_period, output)?;
    if !output[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(period - 1, output[period - 1]));
    }

    for idx in period..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        output[idx] = ema_next_unchecked(data[idx], output[idx - 1], alpha);
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
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
/// [`TechalysisError`] if the period is invalid or if the smoothing factor is invalid.
pub fn period_to_alpha(period: usize, smoothing: Option<Float>) -> Result<Float, TechalysisError> {
    if period == 0 {
        return Err(TechalysisError::BadParam(
            "Period must be greater than 0".to_string(),
        ));
    }

    let smoothing = match smoothing {
        Some(s) => {
            if s <= 0.0 {
                return Err(TechalysisError::BadParam(
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

pub(crate) fn get_alpha_value(
    alpha: Option<Float>,
    period: usize,
) -> Result<Float, TechalysisError> {
    match alpha {
        Some(a) => Ok(a),
        None => period_to_alpha(period, None),
    }
}
