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

use crate::errors::TechalysisError;
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
/// the next values incrementally.
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
/// the SMA calculation.
/// 
/// **Parameters**
/// - `period`: The period used for the SMA calculation, which determines
/// how many values are averaged to compute the SMA.
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
    fn update(&mut self, sample: Float) -> Result<(), TechalysisError> {
        if self.period <= 1 {
            return Err(TechalysisError::BadParam(
                "SMA period must be greater than 1".to_string(),
            ));
        }
        if !sample.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "sample = {sample:?}"
            )));
        }
        if !self.sma.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.sma = {:?}", self.sma
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalysisError::BadParam(
                format!(
                    "SMA state last_window length ({}) does not match period ({})",
                    self.last_window.len(),
                    self.period
                )
            ));
        }

        for (idx, &value) in self.last_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalysisError::DataNonFinite(format!(
                    "window[{idx}] = {value:?}"
                )));
            }
        }

        let mut window = self.last_window.clone();

        let old_value = window
            .pop_front()
            .ok_or(TechalysisError::InsufficientData)?;
        window.push_back(sample);

        let sma = sma_next_unchecked(
            sample,
            old_value,
            self.sma,
            1.0 / (self.period as Float)
        );
        if !sma.is_finite() {
            return Err(TechalysisError::Overflow(0, sma));
        }
        self.sma = sma;
        self.last_window = window;
        
        Ok(())
    }
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
/// or a [`TechalysisError`] error if the calculation fails.
pub fn sma(data: &[Float], period: usize) -> Result<SmaResult, TechalysisError> {
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
/// will be stored.
/// 
/// Returns
/// ---
/// A `Result` containing a [`SmaState`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn sma_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<SmaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / (period as Float);
    if period == 0 || period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if output.len() < len {
        return Err(TechalysisError::BadParam(
            "Output array must be at least as long as the input data array".to_string(),
        ));
    }

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
        output[idx] =
            sma_next_unchecked(data[idx], data[idx - period], output[idx - 1], inv_period);
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
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
) -> Result<Float, TechalysisError> {
    let mut sum: Float = 0.0;
    for idx in 0..period {
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        } else {
            sum += value;
        }
        output[idx] = Float::NAN;
    }
    Ok(sum * inv_period)
}
