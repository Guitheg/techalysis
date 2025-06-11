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
    Inspired by TA-LIB TEMA implementation
*/

//! Triple Exponential Moving Average (TEMA) implementation

use crate::errors::TechalysisError;
use crate::indicators::dema::{
    dema_next_unchecked, dema_skip_period_unchecked, init_dema_unchecked,
};
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};

use crate::traits::State;
use crate::types::Float;

/// TEMA calculation result
/// ---
/// This struct holds the result and the state ([`TemaState`])
/// of the calculation.
/// 
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated TEMA values.
/// - `state`: A [`TemaState`], which can be used to calculate
/// the next values incrementally.
#[derive(Debug)]
pub struct TemaResult {
    /// The calculated TEMA values.
    pub values: Vec<Float>,
    /// A [`TemaState`], which can be used to calculate the next values
    /// incrementally.
    pub state: TemaState,
}

/// TEMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
/// 
/// Attributes
/// ---
/// **Last outputs values**
/// - `tema`: The last calculated Triple Exponential Moving Average (TEMA) value.
/// 
/// **State values**
/// - `ema_1`: The last calculated Exponential Moving Average (EMA) value for
/// the EMA.
/// - `ema_2`: The last calculated Exponential Moving Average (EMA) value for
/// the EMA2.
/// - `ema_3`: The last calculated Exponential Moving Average (EMA) value for
/// the EMA3.
/// 
/// **Parameters**
/// - `period`: The period used for the TEMA calculation.
/// - `alpha`: The alpha factor used in the TEMA calculation,
/// which is traditionally calculated as `smoothing / (period + 1)`.
#[derive(Debug, Clone, Copy)]
pub struct TemaState {
    // Outputs
    /// The last calculated Triple Exponential Moving Average (TEMA) value.
    pub tema: Float,

    // State values
    /// The last calculated Exponential Moving Average (EMA) value for the EMA.
    pub ema_1: Float,
    /// The last calculated Exponential Moving Average (EMA) value for the EMA2.
    pub ema_2: Float,
    /// The last calculated Exponential Moving Average (EMA) value for the EMA3.
    pub ema_3: Float,

    // Parameters
    /// The period used for the TEMA calculation.
    pub period: usize,
    /// The alpha factor used in the TEMA calculation,
    /// which is traditionally calculated as `smoothing / (period + 1)`.
    pub alpha: Float,
}

impl State<Float> for TemaState {
    /// Update the [`TemaState`] with a new sample
    /// 
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the TEMA state
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

        if !self.ema_1.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema_1 = {:?}", self.ema_1
            )));
        }
        if !self.ema_2.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema_2 = {:?}", self.ema_2
            )));
        }
        if !self.ema_3.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema_3 = {:?}", self.ema_3
            )));
        }
        if !self.alpha.is_finite() {
            return Err(TechalysisError::BadParam(format!("self.alpha = {:?}", self.alpha)));
        }

        let (tema, ema_1, ema_2, ema_3) =
            tema_next_unchecked(sample, self.ema_1, self.ema_2, self.ema_3, self.alpha);
        if !tema.is_finite() {
            return Err(TechalysisError::Overflow(0, tema));
        }
        self.tema = tema;
        self.ema_1 = ema_1;
        self.ema_2 = ema_2;
        self.ema_3 = ema_3;
        Ok(())
    }
}

/// Calculation of the TEMA function
/// ---
/// It returns a [`TemaResult`]
/// 
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// 
/// Returns
/// ---
/// A `Result` containing a [`TemaResult`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn tema(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<TemaResult, TechalysisError> {
    let mut output = vec![0.0; data.len()];

    let tema_state = tema_into(data, period, alpha, &mut output)?;

    Ok(TemaResult {
        values: output,
        state: tema_state,
    })
}

/// Calculation of the TEMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`TemaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the TEMA calculation.
/// - `alpha`: An optional alpha value for the TEMA calculation.
/// Used by the inner EMA calculation.
/// 
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated TEMA values
/// will be stored.
/// 
/// Returns
/// ---
/// A `Result` containing a [`TemaState`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn tema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<TemaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    let skip_period = tema_skip_period_unchecked(period);

    if period == 0 || len < skip_period + 1 {
        return Err(TechalysisError::InsufficientData);
    }

    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "EMA period must be greater than 1".to_string(),
        ));
    }

    let alpha = match alpha {
        Some(alpha) => alpha,
        None => period_to_alpha(period, None)?,
    };
    let (output_value, mut ema_1, mut ema_2, mut ema_3) =
        init_tema_unchecked(data, period, inv_period, skip_period, alpha, output)?;
    output[skip_period] = output_value;
    if !output[skip_period].is_finite() {
        return Err(TechalysisError::Overflow(skip_period, output[skip_period]));
    }

    for idx in skip_period + 1..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }

        (output[idx], ema_1, ema_2, ema_3) =
            tema_next_unchecked(data[idx], ema_1, ema_2, ema_3, alpha);

        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }

    Ok(TemaState {
        tema: output[len - 1],
        ema_1,
        ema_2,
        ema_3,
        period,
        alpha,
    })
}

#[inline(always)]
fn tema_next_unchecked(
    new_value: Float,
    prev_ema_1: Float,
    prev_ema_2: Float,
    prev_ema_3: Float,
    alpha: Float,
) -> (Float, Float, Float, Float) {
    let ema_1 = ema_next_unchecked(new_value, prev_ema_1, alpha);
    let ema_2 = ema_next_unchecked(ema_1, prev_ema_2, alpha);
    let ema_3 = ema_next_unchecked(ema_2, prev_ema_3, alpha);
    (calculate_tema(ema_1, ema_2, ema_3), ema_1, ema_2, ema_3)
}

#[inline(always)]
pub(crate) fn init_tema_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    skip_period: usize,
    alpha: Float,
    output: &mut [Float],
) -> Result<(Float, Float, Float, Float), TechalysisError> {
    let dema_skip_period = dema_skip_period_unchecked(period);
    let (_, mut ema_1, mut ema_2) =
        init_dema_unchecked(data, period, inv_period, dema_skip_period, alpha, output)?;
    output[dema_skip_period] = Float::NAN;

    let mut sum_ema_3 = ema_2;
    for idx in dema_skip_period + 1..skip_period {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        (_, ema_1, ema_2) = dema_next_unchecked(data[idx], ema_1, ema_2, alpha);
        sum_ema_3 += ema_2;
        output[idx] = Float::NAN;
    }
    (_, ema_1, ema_2) = dema_next_unchecked(data[skip_period], ema_1, ema_2, alpha);
    sum_ema_3 += ema_2;
    let ema_3 = sum_ema_3 * inv_period;

    Ok((calculate_tema(ema_1, ema_2, ema_3), ema_1, ema_2, ema_3))
}

#[inline(always)]
fn calculate_tema(ema_1: Float, ema_2: Float, ema_3: Float) -> Float {
    (3.0 * ema_1) - (3.0 * ema_2) + ema_3
}

/// Calculate the period to skip for TEMA.
/// 
/// Also known as the "lookback period"
pub fn tema_skip_period_unchecked(period: usize) -> usize {
    3 * (period - 1)
}
