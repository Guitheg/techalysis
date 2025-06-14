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
    Inspired by TA-LIB T3 implementation
*/

//! Tillson Triple Exponential Moving Average (T3) implementation

use crate::errors::TechalibError;
use crate::indicators::ema::{ema_next_unchecked, get_alpha_value};
use crate::indicators::sma::init_sma_unchecked;
use crate::traits::State;
use crate::types::Float;

/// T3 calculation result
/// ---
/// This struct holds the result and the state ([`T3State`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated T3 values.
/// - `state`: A [`T3State`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct T3Result {
    /// The calculated T3 values.
    pub values: Vec<Float>,
    /// The [`T3State`] state of the T3 calculation.
    pub state: T3State,
}

/// T3 calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `t3`: The last calculated T3 value.
///
/// **State values**
/// - `ema1`: The last calculated EMA1 value.
/// - `ema2`: The last calculated EMA2 value.
/// - `ema3`: The last calculated EMA3 value.
/// - `ema4`: The last calculated EMA4 value.
/// - `ema5`: The last calculated EMA5 value.
///
/// **Parameters**
/// - `period`: The period used for the T3 calculation.
/// - `volume_factor`: The volume_factor used for the T3 calculation.
#[derive(Debug, Clone, Copy)]
pub struct T3State {
    // Outputs
    /// The last calculated T3 value.
    pub t3: Float,

    // State values
    /// The last calculated EMA values.
    pub ema_values: T3EmaValues,

    // Parameters
    /// The period used for the T3 calculation.
    pub period: usize,
    /// The alpha value used for the T3 calculation.
    pub alpha: Float,
    /// The volume_factor used for the T3 calculation.
    /// The volume_factor is a smoothing factor that affects the responsiveness of the T3.
    /// It is typically set to a value between 0.0 and 1.0.
    /// A common value is 0.7, but it can be adjusted based on the desired sensitivity.
    pub volume_factor: Float,

    /// The T3 coefficients used in the T3 calculation.
    /// These coefficients are derived from the volume_factor and are used in the T3 calculation.
    pub t3_coefficients: T3Coefficients,
}

/// Coefficients used in the T3 calculation
///
/// These coefficients are derived from the volume_factor and are used in the T3 calculation.
#[derive(Debug, Clone, Copy)]
pub struct T3Coefficients {
    /// The c1 coefficient used in the T3 calculation: `c1 = -volume_factor³`
    pub c1: Float,
    /// The c2 coefficient used in the T3 calculation: `c2 = 3 * volume_factor² + 3 * volume_factor³`
    pub c2: Float,
    /// The c3 coefficient used in the T3 calculation: `c3 = - 3 * volume_factor - 6 * volume_factor² - 3 * volume_factor³`
    pub c3: Float,
    /// The c4 coefficient used in the T3 calculation: `c4 = 1 + 3 * volume_factor + 3 * volume_factor² + volume_factor³`
    pub c4: Float,
}

impl T3Coefficients {
    fn new(volume_factor: Float) -> Self {
        let vfactor_square = volume_factor.powi(2);
        let vfactor_cube = volume_factor.powi(3);
        T3Coefficients {
            c1: -vfactor_cube,
            c2: 3.0 * vfactor_square + 3.0 * vfactor_cube,
            c3: -3.0 * volume_factor - 6.0 * vfactor_square - 3.0 * vfactor_cube,
            c4: 1.0 + 3.0 * volume_factor + 3.0 * vfactor_square + vfactor_cube,
        }
    }
}

/// Ema values used in the T3 calculation
///
/// These values are used to store the last calculated EMA values
#[derive(Debug, Clone, Copy)]
pub struct T3EmaValues {
    /// Represent the 1st order EMA value
    pub ema1: Float,
    /// Represent the 2nd order EMA value
    pub ema2: Float,
    /// Represent the 3rd order EMA value
    pub ema3: Float,
    /// Represent the 4th order EMA value
    pub ema4: Float,
    /// Represent the 5th order EMA value
    pub ema5: Float,
    /// Represent the 6th order EMA value
    pub ema6: Float,
}

impl State<Float> for T3State {
    /// Update the [`T3State`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the T3 state
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_finite(self.t3, "t3")?;
        TechalibError::check_finite(sample, "sample")?;
        if !self.ema_values.ema1.is_finite()
            || !self.ema_values.ema2.is_finite()
            || !self.ema_values.ema3.is_finite()
            || !self.ema_values.ema4.is_finite()
            || !self.ema_values.ema5.is_finite()
            || !self.ema_values.ema6.is_finite()
        {
            return Err(TechalibError::DataNonFinite(format!(
                "ema1 = {}, ema2 = {}, ema3 = {}, ema4 = {}, ema5 = {}, ema6 = {}",
                self.ema_values.ema1,
                self.ema_values.ema2,
                self.ema_values.ema3,
                self.ema_values.ema4,
                self.ema_values.ema5,
                self.ema_values.ema6
            )));
        }

        if !self.t3_coefficients.c1.is_finite()
            || !self.t3_coefficients.c2.is_finite()
            || !self.t3_coefficients.c3.is_finite()
            || !self.t3_coefficients.c4.is_finite()
        {
            return Err(TechalibError::BadParam(format!(
                "c1 = {}, c2 = {}, c3 = {}, c4 = {}",
                self.t3_coefficients.c1,
                self.t3_coefficients.c2,
                self.t3_coefficients.c3,
                self.t3_coefficients.c4
            )));
        }

        if !self.period <= 1 {
            return Err(TechalibError::BadParam(format!(
                "Period must be greater than 1, got: {}",
                self.period
            )));
        }

        if !self.volume_factor.is_finite() || self.volume_factor < 0.0 || self.volume_factor > 1.0 {
            return Err(TechalibError::BadParam(format!(
                "Volume factor must be between 0.0 and 1.0, got: {}",
                self.volume_factor
            )));
        }

        let t3 = t3_next_unchecked(
            sample,
            &mut self.ema_values,
            &self.t3_coefficients,
            self.alpha,
        );

        TechalibError::check_overflow(t3)?;

        self.t3 = t3;
        // ema values update in place (no need to reassign)

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
    Ok(6 * (period - 1))
}

/// Calculation of the T3 function
/// ---
/// It returns a [`T3Result`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period used for the T3 calculation.
/// - `volume_factor`: The volume factor used for the T3 calculation.
/// - `alpha`: The alpha value used for the T3 calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`T3Result`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn t3(
    data: &[Float],
    period: usize,
    volume_factor: Float,
    alpha: Option<Float>,
) -> Result<T3Result, TechalibError> {
    let mut output = vec![0.0; data.len()];

    let t3_state = t3_into(data, period, volume_factor, alpha, output.as_mut_slice())?;

    Ok(T3Result {
        values: output,
        state: t3_state,
    })
}

/// Calculation of the T3 function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`T3State`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period used for the T3 calculation.
/// - `volume_factor`: The volume factor used for the T3 calculation.
/// - `alpha`: The alpha value used for the T3 calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the T3 values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`T3State`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn t3_into(
    data: &[Float],
    period: usize,
    volume_factor: Float,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<T3State, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;
    let len = data.len();
    let lookback = lookback_from_period(period)?;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    if !volume_factor.is_finite() || !(0.0..=1.0).contains(&volume_factor) {
        return Err(TechalibError::BadParam(format!(
            "Volume factor must be between 0.0 and 1.0, got: {}",
            volume_factor
        )));
    }

    let t3_coefficients = T3Coefficients::new(volume_factor);

    let alpha = get_alpha_value(alpha, period)?;

    let (t3, mut t3_ema_values) = init_t3_unchecked(
        data,
        period,
        &t3_coefficients,
        1.0 / period as Float,
        lookback,
        alpha,
        output,
    )?;

    output[lookback] = t3;
    TechalibError::check_overflow_at(lookback, output)?;

    for idx in lookback + 1..len {
        TechalibError::check_finite_at(idx, data)?;
        output[idx] = t3_next_unchecked(data[idx], &mut t3_ema_values, &t3_coefficients, alpha);
        TechalibError::check_overflow_at(idx, output)?;
    }

    Ok(T3State {
        t3: output[len - 1],
        period,
        alpha,
        ema_values: t3_ema_values,
        t3_coefficients,
        volume_factor,
    })
}

#[inline(always)]
fn t3_next_unchecked(
    new_value: Float,
    ema_values: &mut T3EmaValues,
    t3_coefficients: &T3Coefficients,
    alpha: Float,
) -> Float {
    ema_values.ema1 = ema_next_unchecked(new_value, ema_values.ema1, alpha);
    ema_values.ema2 = ema_next_unchecked(ema_values.ema1, ema_values.ema2, alpha);
    ema_values.ema3 = ema_next_unchecked(ema_values.ema2, ema_values.ema3, alpha);
    ema_values.ema4 = ema_next_unchecked(ema_values.ema3, ema_values.ema4, alpha);
    ema_values.ema5 = ema_next_unchecked(ema_values.ema4, ema_values.ema5, alpha);
    ema_values.ema6 = ema_next_unchecked(ema_values.ema5, ema_values.ema6, alpha);
    t3_from_coefficients_unchecked(
        ema_values.ema3,
        ema_values.ema4,
        ema_values.ema5,
        ema_values.ema6,
        t3_coefficients.c1,
        t3_coefficients.c2,
        t3_coefficients.c3,
        t3_coefficients.c4,
    )
}

#[inline(always)]
fn init_t3_unchecked(
    data: &[Float],
    period: usize,
    t3_coefficients: &T3Coefficients,
    inv_period: Float,
    skip_period: usize,
    alpha: Float,
    output: &mut [Float],
) -> Result<(Float, T3EmaValues), TechalibError> {
    // Initialiaztion of ema1
    let mut ema1 = init_sma_unchecked(data, period, inv_period, output)?;

    // Initialiaztion of ema2
    let skip_period_2 = 2 * (period - 1);
    let mut sum_ema2 = ema1;
    for idx in period..=skip_period_2 {
        if !data[idx].is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        ema1 = ema_next_unchecked(data[idx], ema1, alpha);
        sum_ema2 += ema1;
        output[idx] = Float::NAN;
    }
    let mut ema2 = sum_ema2 * inv_period;

    // Initialiaztion of ema3
    let skip_period_3 = 3 * (period - 1);
    let mut sum_ema3 = ema2;
    for idx in skip_period_2 + 1..=skip_period_3 {
        if !data[idx].is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        ema1 = ema_next_unchecked(data[idx], ema1, alpha);
        ema2 = ema_next_unchecked(ema1, ema2, alpha);
        sum_ema3 += ema2;
        output[idx] = Float::NAN;
    }
    let mut ema3 = sum_ema3 * inv_period;

    // Initialiaztion of ema4
    let skip_period_4 = 4 * (period - 1);
    let mut sum_ema4 = ema3;
    for idx in skip_period_3 + 1..=skip_period_4 {
        if !data[idx].is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        ema1 = ema_next_unchecked(data[idx], ema1, alpha);
        ema2 = ema_next_unchecked(ema1, ema2, alpha);
        ema3 = ema_next_unchecked(ema2, ema3, alpha);
        sum_ema4 += ema3;
        output[idx] = Float::NAN;
    }
    let mut ema4 = sum_ema4 * inv_period;

    // Initialization of ema5
    let skip_period_5 = 5 * (period - 1);
    let mut sum_ema5 = ema4;
    for idx in skip_period_4 + 1..=skip_period_5 {
        if !data[idx].is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        ema1 = ema_next_unchecked(data[idx], ema1, alpha);
        ema2 = ema_next_unchecked(ema1, ema2, alpha);
        ema3 = ema_next_unchecked(ema2, ema3, alpha);
        ema4 = ema_next_unchecked(ema3, ema4, alpha);
        sum_ema5 += ema4;
        output[idx] = Float::NAN;
    }
    let mut ema5 = sum_ema5 * inv_period;

    // Initialization of ema6
    let mut sum_ema6 = ema5;
    for idx in skip_period_5 + 1..skip_period {
        if !data[idx].is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        ema1 = ema_next_unchecked(data[idx], ema1, alpha);
        ema2 = ema_next_unchecked(ema1, ema2, alpha);
        ema3 = ema_next_unchecked(ema2, ema3, alpha);
        ema4 = ema_next_unchecked(ema3, ema4, alpha);
        ema5 = ema_next_unchecked(ema4, ema5, alpha);
        sum_ema6 += ema5;
        output[idx] = Float::NAN;
    }
    ema1 = ema_next_unchecked(data[skip_period], ema1, alpha);
    ema2 = ema_next_unchecked(ema1, ema2, alpha);
    ema3 = ema_next_unchecked(ema2, ema3, alpha);
    ema4 = ema_next_unchecked(ema3, ema4, alpha);
    ema5 = ema_next_unchecked(ema4, ema5, alpha);
    sum_ema6 += ema5;
    let ema6 = sum_ema6 * inv_period;

    Ok((
        t3_from_coefficients_unchecked(
            ema3,
            ema4,
            ema5,
            ema6,
            t3_coefficients.c1,
            t3_coefficients.c2,
            t3_coefficients.c3,
            t3_coefficients.c4,
        ),
        T3EmaValues {
            ema1,
            ema2,
            ema3,
            ema4,
            ema5,
            ema6,
        },
    ))
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn t3_from_coefficients_unchecked(
    ema3: Float,
    ema4: Float,
    ema5: Float,
    ema6: Float,
    c1: Float,
    c2: Float,
    c3: Float,
    c4: Float,
) -> Float {
    c1 * ema6 + c2 * ema5 + c3 * ema4 + c4 * ema3
}

#[cfg(test)]
mod tests {
    use crate::indicators::ema::period_to_alpha;

    use super::*;

    #[test]
    fn init_t3_unchecked_nominal_case() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0,
        ];
        let period = 5;
        let output = vec![0.0; data.len()];
        let t3_coefficients = T3Coefficients::new(0.7);
        let expected_t3 = 23.2; // Expected T3 value for the given data and period
        let alpha = period_to_alpha(period, None).unwrap();

        let (t3, _) = init_t3_unchecked(
            &data,
            period,
            &t3_coefficients,
            1.0 / period as Float,
            lookback_from_period(period).unwrap(),
            alpha,
            &mut output.clone(),
        )
        .unwrap();

        assert!(t3.is_finite());
        assert!(
            t3 - expected_t3 < 1e-8,
            "Expected: {}, got: {}",
            expected_t3,
            t3
        );
    }
}
