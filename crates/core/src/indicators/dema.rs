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
    Inspired by TA-LIB DEMA implementation
*/

//! Double Exponential Moving Average (DEMA) implementation

use crate::errors::TechalibError;
use crate::indicators::ema::{ema_next_unchecked, get_alpha_value};
use crate::indicators::sma::init_sma_unchecked;
use crate::traits::State;
use crate::types::Float;

/// Double Exponential Moving Average (DEMA) result.
/// ---
/// This struct holds the result of the Bollinger Bands calculation.
/// It contains the upper, middle, and lower bands as well as the state of the calculation.
///
/// Attributes
/// ---
/// - `values`: The calculated DEMA values.
/// - `state`: A [`DemaState`], which can be used to calculate the next values
///   incrementally.
#[derive(Debug)]
pub struct DemaResult {
    /// The calculated DEMA values.
    pub values: Vec<Float>,
    /// A [`DemaState`], which can be used to calculate the next values
    /// incrementally.
    pub state: DemaState,
}

/// DEMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `dema`: The last calculated DEMA value.
///
/// **State values**
/// - `ema_1`: The last calculated EMA value.
/// - `ema_2`: The last calculated EMA2 value.
///
/// **Parameters**
/// - `period`: The period used for the DEMA calculation.
/// - `alpha`: The alpha factor used for the EMA calculation.
#[derive(Debug, Clone, Copy)]
pub struct DemaState {
    // Outputs values
    /// The last calculated DEMA value
    pub dema: Float,

    // State values
    /// The last calculated EMA value
    pub ema_1: Float,
    /// The last calculated EMA2 value
    pub ema_2: Float,

    // Parameters
    /// The period used for the DEMA calculation
    pub period: usize,
    /// The alpha factor used for the EMA calculation
    pub alpha: Float,
}

impl State<Float> for DemaState {
    /// Update the [`DemaState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input value to update the state with.
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_period(self.period)?;
        TechalibError::check_finite(sample, "sample")?;
        TechalibError::check_finite(self.alpha, "alpha")?;
        TechalibError::check_finite(self.ema_1, "ema_1")?;
        TechalibError::check_finite(self.ema_2, "ema_2")?;

        let (dema, ema_1, ema_2) = dema_next_unchecked(sample, self.ema_1, self.ema_2, self.alpha);
        TechalibError::check_overflow(dema)?;

        self.dema = dema;
        self.ema_1 = ema_1;
        self.ema_2 = ema_2;
        Ok(())
    }
}

/// Lookback period for DEMA calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n` first values that will be return will be `NaN`
/// and the next values will be the DEMA values.
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    TechalibError::check_period(period)?;
    Ok(2 * (period - 1))
}

/// Calculation of the DEMA function
/// ---
/// It returns a [`DemaResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] values representing the input data.
///
/// Returns
/// ---
/// A `Result` containing a [`DemaResult`] with the calculated DEMA values and state,
/// or a [`TechalibError`] error if the calculation fails.
pub fn dema(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<DemaResult, TechalibError> {
    let mut output = vec![0.0; data.len()];

    let dema_state = dema_into(data, period, alpha, &mut output)?;

    Ok(DemaResult {
        values: output,
        state: dema_state,
    })
}

/// Calculation of the DEMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`DemaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] values representing the input data.
/// - `period`: The period for the DEMA calculation.
/// - `alpha`: An optional alpha value for the EMA calculation. If `None`, it will be calculated
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the DEMA values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`DemaState`]
/// or a [`TechalibError`] error if the calculation fails.
pub fn dema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<DemaState, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    let lookback = lookback_from_period(period)?;

    if period == 0 || len < lookback + 1 {
        return Err(TechalibError::InsufficientData);
    }

    let alpha = get_alpha_value(alpha, period)?;
    let (output_value, mut ema_1, mut ema_2) =
        init_dema_unchecked(data, period, inv_period, lookback, alpha, output)?;
    output[lookback] = output_value;
    TechalibError::check_overflow_at(lookback, output)?;

    for idx in lookback + 1..len {
        TechalibError::check_finite_at(idx, data)?;

        (output[idx], ema_1, ema_2) = dema_next_unchecked(data[idx], ema_1, ema_2, alpha);

        TechalibError::check_overflow_at(idx, output)?;
    }

    Ok(DemaState {
        dema: output[len - 1],
        ema_1,
        ema_2,
        period,
        alpha,
    })
}

#[inline(always)]
pub(crate) fn dema_next_unchecked(
    new_value: Float,
    prev_ema_1: Float,
    prev_ema_2: Float,
    alpha: Float,
) -> (Float, Float, Float) {
    let ema_1 = ema_next_unchecked(new_value, prev_ema_1, alpha);
    let ema_2 = ema_next_unchecked(ema_1, prev_ema_2, alpha);
    (calculate_dema(ema_1, ema_2), ema_1, ema_2)
}

#[inline(always)]
pub(crate) fn init_dema_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    skip_period: usize,
    alpha: Float,
    output: &mut [Float],
) -> Result<(Float, Float, Float), TechalibError> {
    let mut ema_1 = init_sma_unchecked(data, period, inv_period, output)?;

    let mut sum_ema_2 = ema_1;
    for idx in period..skip_period {
        TechalibError::check_finite_at(idx, data)?;
        ema_1 = ema_next_unchecked(data[idx], ema_1, alpha);
        sum_ema_2 += ema_1;
        output[idx] = Float::NAN;
    }
    TechalibError::check_finite_at(skip_period, data)?;
    ema_1 = ema_next_unchecked(data[skip_period], ema_1, alpha);
    sum_ema_2 += ema_1;
    let ema_2 = sum_ema_2 * inv_period;

    Ok((calculate_dema(ema_1, ema_2), ema_1, ema_2))
}

#[inline(always)]
fn calculate_dema(ema_1: Float, ema_2: Float) -> Float {
    (2.0 * ema_1) - ema_2
}
