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
    Inspired by TA-LIB KAMA implementation
*/

//! Kaufman Adaptive Moving Average (KAMA) implementation

use std::collections::VecDeque;

use crate::errors::TechalysisError;
use crate::indicators::ema::ema_next_unchecked;
use crate::traits::State;
use crate::types::Float;

/// Fast period for KAMA calculation. It is used to calculate the Smoothing Constant (SC).
pub const FAST_PERIOD: Float = 2.0;
/// Slow period for KAMA calculation. It is used to calculate the Smoothing Constant (SC).
pub const SLOW_PERIOD: Float = 30.0;

const SC_SLOW: Float = 2.0 / (SLOW_PERIOD + 1.0);
const SC_DELTA: Float = (2.0 / (FAST_PERIOD + 1.0)) - SC_SLOW;

/// KAMA calculation result
/// ---
/// This struct holds the result and the state ([`KamaState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] containing the calculated KAMA values.
/// - `state`: A [`KamaState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct KamaResult {
    /// The calculated KAMA values.
    pub values: Vec<Float>,
    /// The [`KamaState`] state of the KAMA calculation.
    pub state: KamaState,
}

/// KAMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `kama`: The last calculated KAMA value.
///
/// **State values**
/// - `roc_sum`: The last calculated Efficiency Ratio sum, defined as `roc_sum = roc1 + roc_sum - prev_roc1`
///   or `sum of |data[t-i] - data[t-i-1]| for i in [1, period]`.
/// - `last_window`: The last window containing the previous input value over the last period.
/// - `trailing_value`: The last trailing value used in the calculation.
///
/// **Parameters**
/// - `period`: The period used for the KAMA calculation to calculate Efficiency Ratio.
#[derive(Debug, Clone)]
pub struct KamaState {
    // Outputs
    /// The last calculated KAMA value.
    pub kama: Float,

    // State values
    /// The last calculated Efficiency Ratio sum.
    /// Define such as: `roc_sum = roc1 + roc_sum - prev_roc1` or `sum of |data[t-i] - data[t-i-1]| for i in [1, period]`
    pub roc_sum: Float,
    /// The last window containing the previous input value over the last period.
    pub last_window: VecDeque<Float>,
    /// The last trailing value used in the calculation.
    /// It is the value that was removed from the `last_window` during the last update.
    pub trailing_value: Float,

    // Parameters
    /// The period used for the KAMA calculation to calculate Efficiency Ratio.
    pub period: usize,
}

impl State<Float> for KamaState {
    /// Update the [`KamaState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the KAMA state
    fn update(&mut self, sample: Float) -> Result<(), TechalysisError> {
        if !sample.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "sample = {sample:?}",
            )));
        }

        if self.period <= 1 {
            return Err(TechalysisError::BadParam(
                "KAMA period must be greater than 1".to_string(),
            ));
        }
        if !self.kama.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "kama = {kama:?}",
                kama = self.kama
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalysisError::BadParam(format!(
                "KAMA state last_window length must be equal to period ({}), got: {}",
                self.period,
                self.last_window.len()
            )));
        }

        for (idx, &value) in self.last_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalysisError::DataNonFinite(format!(
                    "window[{idx}] = {value:?}"
                )));
            }
        }

        let mut window = self.last_window.clone();

        let new_trailing_value = window
            .pop_front()
            .ok_or(TechalysisError::InsufficientData)?;
        let prev_value = *window.back().ok_or(TechalysisError::InsufficientData)?;
        window.push_back(sample);

        let (kama, roc_sum) = kama_next_unchecked(
            sample,
            prev_value,
            new_trailing_value,
            (new_trailing_value - self.trailing_value).abs(),
            self.roc_sum,
            self.kama,
        );

        if !kama.is_finite() {
            return Err(TechalysisError::Overflow(0, kama));
        }
        self.kama = kama;
        self.roc_sum = roc_sum;
        self.last_window = window;
        self.trailing_value = new_trailing_value;

        Ok(())
    }
}

/// Lookback period for ${INDICATORNAME} calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n` first values that will be return will be `NaN`
/// and the next values will be the KAMA values.
#[inline(always)]
pub fn lookback_from_period(period: usize) -> usize {
    period
}

/// Calculation of the KAMA function
/// ---
/// It returns a [`KamaResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] containing the input data for the KAMA calculation.
/// - `period`: The period used for the KAMA calculation to calculate Efficiency Ratio.
///
/// Returns
/// ---
/// A `Result` containing a [`KamaResult`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn kama(data: &[Float], period: usize) -> Result<KamaResult, TechalysisError> {
    let mut output = vec![0.0; data.len()];

    let kama_state = kama_into(data, period, output.as_mut_slice())?;

    Ok(KamaResult {
        values: output,
        state: kama_state,
    })
}

/// Calculation of the KAMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`KamaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] containing the input data for the KAMA calculation.
/// - `period`: The period used for the KAMA calculation to calculate Efficiency Ratio.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the KAMA values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`KamaState`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn kama_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<KamaState, TechalysisError> {
    let len = data.len();
    let lookback = lookback_from_period(period);

    if len <= lookback {
        return Err(TechalysisError::InsufficientData);
    }

    if period <= 1 {
        return Err(TechalysisError::BadParam(format!(
            "Period must be greater than 1, got: {}",
            period
        )));
    }

    let (kama, mut roc_sum) = init_kama_unchecked(data, lookback, output)?;

    if !kama.is_finite() {
        return Err(TechalysisError::Overflow(lookback, output[lookback]));
    }
    output[lookback] = kama;

    for idx in lookback + 1..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{}] = {:?}",
                idx, data[idx]
            )));
        }

        (output[idx], roc_sum) = kama_next_unchecked(
            data[idx],
            data[idx - 1],
            data[idx - period],
            (data[idx - period] - data[idx - period - 1]).abs(),
            roc_sum,
            output[idx - 1],
        );

        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }

    Ok(KamaState {
        kama: output[len - 1],
        roc_sum,
        last_window: VecDeque::from(data[len - period..len].to_vec()),
        trailing_value: data[len - period - 1],
        period,
    })
}

#[inline(always)]
fn init_kama_unchecked(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<(Float, Float), TechalysisError> {
    let mut roc_sum = 0.0;
    output[0] = Float::NAN;

    if !data[0].is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "data[{}] = {:?}",
            0, data[0]
        )));
    }
    if !data[1].is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "data[{}] = {:?}",
            1, data[1]
        )));
    }
    let first_roc1 = (data[1] - data[0]).abs();
    roc_sum += first_roc1;
    output[1] = Float::NAN;

    for idx in 2..period {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{}] = {:?}",
                idx, data[idx]
            )));
        }
        roc_sum += (data[idx] - data[idx - 1]).abs();
        output[idx] = Float::NAN;
    }
    if !data[period].is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "data[{}] = {:?}",
            period, data[period]
        )));
    }
    roc_sum += (data[period] - data[period - 1]).abs();
    let prev_kama = data[period - 1];
    let period_roc = (data[period] - data[0]).abs();

    let er = if roc_sum < period_roc {
        1.0
    } else {
        period_roc / roc_sum
    };
    let mut sc = er * SC_DELTA + SC_SLOW;
    sc *= sc;

    Ok((ema_next_unchecked(data[period], prev_kama, sc), roc_sum))
}

#[inline(always)]
fn kama_next_unchecked(
    new_value: Float,
    prev_value: Float,
    trailing_value: Float,
    trailing_roc: Float,
    roc_sum: Float,
    prev_kama: Float,
) -> (Float, Float) {
    let roc1 = (new_value - prev_value).abs();
    let diff = (new_value - trailing_value).abs();
    let roc_sum = roc_sum - trailing_roc + roc1;
    let er = if roc_sum <= diff { 1.0 } else { diff / roc_sum };

    let mut sc = er * SC_DELTA + SC_SLOW;
    sc *= sc;
    (ema_next_unchecked(new_value, prev_kama, sc), roc_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_kama_unchecked_nominal_case() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let period = 4;
        let mut output = vec![0.0; data.len()];
        let expected_kama = 4.444444;
        let (kama, _) = init_kama_unchecked(&data, period, &mut output).unwrap();
        assert!(
            kama - expected_kama < 1e-6,
            "Expected KAMA to be {}, but got {}",
            expected_kama,
            kama
        );
    }

    #[test]
    fn test_kama_next_unchecked_nominal_case() {
        let data = [
            88.10, 84.69, 84.46, 87.47, 90.14, 87.12, 88.51, 86.08, 86.22, 90.98,
        ];
        let period = 4;
        let mut output = vec![0.0; data.len()];
        let expected_kama = 87.572903;
        let expected_next_kama = [87.549283, 87.639365, 87.603087];
        let (mut kama, mut roc_sum) = init_kama_unchecked(&data, period, &mut output).unwrap();
        assert!(
            (kama - expected_kama).abs() < 1e-6,
            "Init, expected KAMA to be {}, but got {}",
            expected_kama,
            kama
        );
        for t in 0..3 {
            (kama, roc_sum) = kama_next_unchecked(
                data[period + t + 1],
                data[period + t],
                data[t + 1],
                (data[t + 1] - data[t]).abs(),
                roc_sum,
                kama,
            );
            assert!(
                (kama - expected_next_kama[t]).abs() < 1e-6,
                "Next[{}], expected KAMA to be {}, but got {}",
                t,
                expected_next_kama[t],
                kama
            );
        }
    }
}
