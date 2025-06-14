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

use crate::errors::TechalibError;
use crate::indicators::dema::{
    dema_next_unchecked, init_dema_unchecked, lookback_from_period as dema_lookback_from_period,
};
use crate::indicators::ema::{ema_next_unchecked, get_alpha_value};

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
///   the next values incrementally.
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
///   the EMA.
/// - `ema_2`: The last calculated Exponential Moving Average (EMA) value for
///   the EMA2.
/// - `ema_3`: The last calculated Exponential Moving Average (EMA) value for
///   the EMA3.
///
/// **Parameters**
/// - `period`: The period used for the TEMA calculation.
/// - `alpha`: The alpha factor used in the TEMA calculation,
///   which is traditionally calculated as `smoothing / (period + 1)`.
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
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_period(self.period)?;
        TechalibError::check_finite(sample, "sample")?;
        TechalibError::check_finite(self.ema_1, "ema_1")?;
        TechalibError::check_finite(self.ema_2, "ema_2")?;
        TechalibError::check_finite(self.ema_3, "ema_3")?;
        TechalibError::check_finite(self.alpha, "alpha")?;

        let (tema, ema_1, ema_2, ema_3) =
            tema_next_unchecked(sample, self.ema_1, self.ema_2, self.ema_3, self.alpha);

        TechalibError::check_overflow(tema)?;

        self.tema = tema;
        self.ema_1 = ema_1;
        self.ema_2 = ema_2;
        self.ema_3 = ema_3;

        Ok(())
    }
}

/// Lookback period for ${INDICATORNAME} calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    TechalibError::check_period(period)?;
    Ok(3 * (period - 1))
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
/// or a [`TechalibError`] error if the calculation fails.
pub fn tema(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<TemaResult, TechalibError> {
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
///   Used by the inner EMA calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated TEMA values
///   will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`TemaState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn tema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<TemaState, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    let lookback = lookback_from_period(period)?;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let alpha = get_alpha_value(alpha, period)?;
    let (output_value, mut ema_1, mut ema_2, mut ema_3) =
        init_tema_unchecked(data, period, inv_period, lookback, alpha, output)?;
    output[lookback] = output_value;
    TechalibError::check_overflow_at(lookback, output)?;

    for idx in lookback + 1..len {
        TechalibError::check_finite_at(idx, data)?;

        (output[idx], ema_1, ema_2, ema_3) =
            tema_next_unchecked(data[idx], ema_1, ema_2, ema_3, alpha);

        TechalibError::check_overflow_at(idx, output)?;
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
pub(crate) fn tema_next_unchecked(
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
    lookback: usize,
    alpha: Float,
    output: &mut [Float],
) -> Result<(Float, Float, Float, Float), TechalibError> {
    let dema_lookback = dema_lookback_from_period(period)?;
    let (_, mut ema_1, mut ema_2) =
        init_dema_unchecked(data, period, inv_period, dema_lookback, alpha, output)?;
    output[dema_lookback] = Float::NAN;

    let mut sum_ema_3 = ema_2;
    for idx in dema_lookback + 1..lookback {
        TechalibError::check_finite_at(idx, data)?;
        (_, ema_1, ema_2) = dema_next_unchecked(data[idx], ema_1, ema_2, alpha);
        sum_ema_3 += ema_2;
        output[idx] = Float::NAN;
    }
    TechalibError::check_finite_at(lookback, data)?;
    (_, ema_1, ema_2) = dema_next_unchecked(data[lookback], ema_1, ema_2, alpha);
    sum_ema_3 += ema_2;
    let ema_3 = sum_ema_3 * inv_period;

    Ok((calculate_tema(ema_1, ema_2, ema_3), ema_1, ema_2, ema_3))
}

#[inline(always)]
fn calculate_tema(ema_1: Float, ema_2: Float, ema_3: Float) -> Float {
    (3.0 * ema_1) - (3.0 * ema_2) + ema_3
}
