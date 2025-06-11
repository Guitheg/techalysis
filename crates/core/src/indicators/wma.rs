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
    Inspired by TA-LIB WMA implementation
*/

//! Weighted Moving Average (WMA) implementation

use crate::errors::TechalysisError;
use crate::traits::State;
use crate::types::Float;
use std::collections::VecDeque;

/// WMA calculation result
/// ---
/// This struct holds the result and the state ([`WmaState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated WMA values.
/// - `state`: A [`WmaState`], which can be used to calculate
/// the next values incrementally.
#[derive(Debug)]
pub struct WmaResult {
    /// The calculated WMA values.
    pub values: Vec<Float>,
    /// A [`WmaState`], which can be used to calculate
    /// the next values incrementally.
    pub state: WmaState,
}

/// WMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `wma`: The last calculated WMA value.
///
/// **State values**
/// - `period_sub`: The sumation to subtract from the period sum.
/// - `period_sum`: The weighted sum of the previous window.
/// - `last_window`: A deque containing the last `period` values used for
/// the WMA calculation.
///
/// **Parameters**
/// - `period`: The period used for the WMA calculation, which determines
/// how many values are averaged to compute the WMA.
#[derive(Debug, Clone)]
pub struct WmaState {
    // Outputs
    /// The last calculated WMA value
    pub wma: Float,

    // State values
    /// The sumation to subtract from the period sum
    pub period_sub: Float,
    /// The weighted sum of the previous window
    pub period_sum: Float,
    /// A deque containing the last `period` values used for
    /// the WMA calculation
    pub last_window: VecDeque<Float>,

    // Parameters
    /// The period used for the WMA calculation, which determines
    pub period: usize,
}

impl State<Float> for WmaState {
    /// Update the [`WmaState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the WMA state
    fn update(&mut self, sample: Float) -> Result<(), TechalysisError> {
        if self.period <= 1 {
            return Err(TechalysisError::BadParam(
                "WMA period must be greater than 1".to_string(),
            ));
        }
        if !sample.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "sample = {sample:?}"
            )));
        }
        if !self.wma.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.wma = {:?}",
                self.wma
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalysisError::BadParam(format!(
                "WMA state window length ({}) does not match period ({})",
                self.last_window.len(),
                self.period
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
        let inv_weight_sum = inv_weight_sum_linear(self.period);

        let old_value = window
            .pop_front()
            .ok_or(TechalysisError::InsufficientData)?;
        window.push_back(sample);

        let (wma, new_period_sub, new_period_sum) = wma_next_unchecked(
            sample,
            old_value,
            self.period as Float,
            self.period_sub,
            self.period_sum,
            inv_weight_sum,
        );

        if !wma.is_finite() {
            return Err(TechalysisError::Overflow(0, wma));
        }

        self.wma = wma;
        self.period_sub = new_period_sub;
        self.period_sum = new_period_sum;
        self.last_window = window;

        Ok(())
    }
}

/// Calculation of the WMA function
/// ---
/// It returns a [`WmaResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the WMA calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`WmaResult`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn wma(data: &[Float], period: usize) -> Result<WmaResult, TechalysisError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let wma_state = wma_into(data, period, &mut output)?;
    Ok(WmaResult {
        values: output,
        state: wma_state,
    })
}

/// Calculation of the WMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`WmaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the WMA calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated WMA values
///
/// Returns
/// ---
/// A `Result` containing a [`WmaState`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn wma_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<WmaState, TechalysisError> {
    let len = data.len();
    let inv_weight_sum = inv_weight_sum_linear(period);
    if period == 0 || period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "WMA period must be greater than 1".to_string(),
        ));
    }

    if output.len() < len {
        return Err(TechalysisError::BadParam(
            "Output array must be at least as long as the input data array".to_string(),
        ));
    }

    let (mut period_sub, mut period_sum) =
        init_wma_unchecked(data, period, inv_weight_sum, output)?;

    for idx in period..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        (output[idx], period_sub, period_sum) = wma_next_unchecked(
            data[idx],
            data[idx - period],
            period as Float,
            period_sub,
            period_sum,
            inv_weight_sum,
        );
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }
    Ok(WmaState {
        wma: output[len - 1],
        period,
        period_sub,
        period_sum,
        last_window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

#[inline(always)]
fn wma_next_unchecked(
    new_value: Float,
    old_value: Float,
    period: Float,
    period_sub: Float,
    period_sum: Float,
    inv_weight_sum: Float,
) -> (Float, Float, Float) {
    let new_period_sub = period_sub - old_value + new_value;
    let new_weighted_sum = period_sum + new_value * period;
    (
        new_weighted_sum * inv_weight_sum,
        new_period_sub,
        new_weighted_sum - new_period_sub,
    )
}

#[inline(always)]
fn init_wma_unchecked(
    data: &[Float],
    period: usize,
    inv_weight_sum: Float,
    output: &mut [Float],
) -> Result<(Float, Float), TechalysisError> {
    let mut period_sub: Float = 0.0;
    let mut period_sum: Float = 0.0;
    for idx in 0..period {
        let weight = idx as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        }
        period_sub += value;
        period_sum += value * weight;
        output[idx] = Float::NAN;
    }
    output[period - 1] = (period_sum + period_sub) * inv_weight_sum;
    if !output[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(period - 1, output[period - 1]));
    }
    Ok((period_sub, period_sum))
}

#[inline(always)]
fn inv_weight_sum_linear(period: usize) -> Float {
    2.0 / (period * (period + 1)) as Float
}
