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
    Inspired by TA-LIB ROC implementation
*/

//! Rate Of Change (ROC) implementation

use std::collections::VecDeque;

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// ROC calculation result
/// ---
/// This struct holds the result and the state ([`RocState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated values.
/// - `state`: A [`RocState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct RocResult {
    /// The calculated ROC values.
    pub values: Vec<Float>,
    /// The [`RocState`] state of the ROC calculation.
    pub state: RocState,
}

/// ROC calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `roc`: The last calculated ROC value.
///
/// **State values**
/// - `last_window`: The last value of the window used for the ROC calculation.
///
/// **Parameters**
/// - `period`: The period for the ROC calculation.
#[derive(Debug, Clone)]
pub struct RocState {
    // Outputs
    /// The last calculated ROC value.
    pub roc: Float,

    // State values
    /// The last value of the window used for the ROC calculation.
    pub last_window: VecDeque<Float>,

    // Parameters
    /// The period used for the ROC calculation.
    pub period: usize,
}

impl State<Float> for RocState {
    /// Update the [`RocState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the ROC state
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_finite(sample, "sample")?;
        if self.period < 1 {
            return Err(TechalibError::BadParam(format!(
                "Period must be greater than 0, got {}",
                self.period
            )));
        }

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

        let new_roc = roc_next_unchecked(sample, old_value);

        TechalibError::check_overflow(new_roc)?;
        self.roc = new_roc;
        self.last_window = window;
        Ok(())
    }
}

/// Lookback period for ROC calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    if period < 1 {
        return Err(TechalibError::BadParam(format!(
            "Period must be greater than 0, got {}",
            period
        )));
    }
    Ok(period)
}

/// Calculation of the ROC function
/// ---
/// It returns a [`RocResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`RocResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn roc(data: &[Float], period: usize) -> Result<RocResult, TechalibError> {
    let mut output = vec![0.0; data.len()];

    let roc_state = roc_into(data, period, output.as_mut_slice())?;

    Ok(RocResult {
        values: output,
        state: roc_state,
    })
}

/// Calculation of the ROC function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`RocState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`RocState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn roc_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<RocState, TechalibError> {
    TechalibError::check_same_length(("output", output), ("data", data))?;

    let len = data.len();
    let lookback = lookback_from_period(period)?;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let output_value = init_roc_unchecked(data, lookback, output)?;
    output[lookback] = output_value;
    TechalibError::check_overflow_at(lookback, output)?;

    for idx in lookback + 1..len {
        TechalibError::check_finite_at(idx, data)?;

        output[idx] = roc_next_unchecked(data[idx], data[idx - period]);

        TechalibError::check_overflow_at(idx, output)?;
    }

    Ok(RocState {
        roc: output[len - 1],
        last_window: VecDeque::from(data[len - period..len].to_vec()),
        period,
    })
}

#[inline(always)]
fn init_roc_unchecked(
    data: &[Float],
    lookback: usize,
    output: &mut [Float],
) -> Result<Float, TechalibError> {
    for (idx, item) in output.iter_mut().enumerate().take(lookback) {
        TechalibError::check_finite_at(idx, data)?;
        *item = Float::NAN;
    }
    TechalibError::check_finite_at(lookback, data)?;
    Ok(roc_next_unchecked(data[lookback], data[0]))
}

#[inline(always)]
fn roc_next_unchecked(new_value: Float, last_value: Float) -> Float {
    ((new_value / last_value) - 1.0) * 100.0
}
