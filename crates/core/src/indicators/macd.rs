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
    Inspired by TA-LIB MACD implementation
*/

//! Moving Average Convergence Divergence (MACD) implementation

use super::ema::period_to_alpha;
use crate::errors::TechalysisError;
use crate::indicators::ema::ema_next_unchecked;
use crate::traits::State;
use crate::types::Float;

/// MACD calculation result
/// ---
/// This struct holds the result and the state ([`MacdState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `macd`: A vector of [`Float`] representing the calculated MACD line values.
/// - `signal`: A vector of [`Float`] representing the calculated signal line values.
/// - `histogram`: A vector of [`Float`] representing the calculated histogram values.
/// - `state`: A [`MacdState`], which can be used to calculate
/// the next values incrementally.
#[derive(Debug)]
pub struct MacdResult {
    /// The calculated MACD line values.
    pub macd: Vec<Float>,
    /// The calculated signal line values.
    pub signal: Vec<Float>,
    /// The calculated histogram values.
    pub histogram: Vec<Float>,
    /// A [`MacdState`], which can be used to calculate the next values
    /// incrementally.
    pub state: MacdState,
}

/// MACD calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `macd`: The last calculated MACD value.
/// - `signal`: The last calculated signal line value.
/// - `histogram`: The last calculated histogram value.
///
/// **State values**
/// - `fast_ema`: The last calculated fast Exponential Moving Average (EMA) value.
/// - `slow_ema`: The last calculated slow Exponential Moving Average (EMA) value.
///
/// **Parameters**
/// - `fast_period`: The period used for the fast EMA calculation.
/// - `slow_period`: The period used for the slow EMA calculation.
/// - `signal_period`: The period used for the signal line calculation.
#[derive(Debug, Clone, Copy)]
pub struct MacdState {
    // Outputs values
    /// The last calculated MACD value
    pub macd: Float,
    /// The last calculated signal line value
    pub signal: Float,
    /// The last calculated histogram value
    pub histogram: Float,

    // State values
    /// The last calculated fast Exponential Moving Average (EMA) value
    pub fast_ema: Float,
    /// The last calculated slow Exponential Moving Average (EMA) value
    pub slow_ema: Float,

    // Parameters
    /// The period used for the fast EMA calculation
    pub fast_period: usize,
    /// The period used for the slow EMA calculation
    pub slow_period: usize,
    /// The period used for the signal line calculation
    pub signal_period: usize,
}

impl State<Float> for MacdState {
    /// Update the [`MacdState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the MACD state.
    fn update(&mut self, sample: Float) -> Result<(), TechalysisError> {
        if self.fast_period >= self.slow_period {
            return Err(TechalysisError::BadParam(
                "Fast period must be less than slow period".to_string(),
            ));
        }

        let fast_alpha = period_to_alpha(self.fast_period, None)?;
        let slow_alpha = period_to_alpha(self.slow_period, None)?;
        let signal_alpha = period_to_alpha(self.signal_period, None)?;

        if !sample.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "sample = {sample:?}",
            )));
        }
        if !self.fast_ema.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.fast_ema = {:?}",
                self.fast_ema
            )));
        }
        if !self.slow_ema.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.slow_ema = {:?}",
                self.slow_ema
            )));
        }
        if !self.signal.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.signal = {:?}",
                self.signal
            )));
        }
        if self.fast_period <= 1 {
            return Err(TechalysisError::BadParam(
                "Fast period must be greater than 1".to_string(),
            ));
        }
        if self.slow_period <= 1 {
            return Err(TechalysisError::BadParam(
                "Slow period must be greater than 1".to_string(),
            ));
        }
        if self.signal_period <= 1 {
            return Err(TechalysisError::BadParam(
                "Signal period must be greater than 1".to_string(),
            ));
        }

        let (fast_ema, slow_ema, macd, signal, histogram) = macd_next_unchecked(
            sample,
            self.fast_ema,
            self.slow_ema,
            self.signal,
            fast_alpha,
            slow_alpha,
            signal_alpha,
        );

        if !macd.is_finite() {
            return Err(TechalysisError::Overflow(0, macd));
        }
        if !signal.is_finite() {
            return Err(TechalysisError::Overflow(0, signal));
        }
        if !histogram.is_finite() {
            return Err(TechalysisError::Overflow(0, histogram));
        }

        self.fast_ema = fast_ema;
        self.slow_ema = slow_ema;
        self.macd = macd;
        self.signal = signal;
        self.histogram = histogram;

        Ok(())
    }
}

/// Calculation of the MACD function
/// ---
/// It returns a [`MacdResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `fast_period`: The period for the fast EMA calculation.
/// - `slow_period`: The period for the slow EMA calculation.
/// - `signal_period`: The period for the signal line calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`MacdResult`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn macd(
    data: &[Float],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<MacdResult, TechalysisError> {
    let size: usize = data.len();

    let mut output_macd = vec![0.0; size];
    let mut output_signal = vec![0.0; size];
    let mut output_histogram = vec![0.0; size];

    let macd_state = macd_into(
        data,
        fast_period,
        slow_period,
        signal_period,
        &mut output_macd,
        &mut output_signal,
        &mut output_histogram,
    )?;

    Ok(MacdResult {
        macd: output_macd,
        signal: output_signal,
        histogram: output_histogram,
        state: macd_state,
    })
}

/// Calculation of the MACD function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`MacdState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `fast_period`: The period for the fast EMA calculation.
/// - `slow_period`: The period for the slow EMA calculation.
/// - `signal_period`: The period for the signal line calculation.
///
/// Output Arguments
/// ---
/// - `output_macd`: A mutable slice of [`Float`] where the calculated MACD
/// values will be stored.
/// - `output_signal`: A mutable slice of [`Float`] where the calculated signal
/// line values will be stored.
/// - `output_histogram`: A mutable slice of [`Float`] where the calculated
/// histogram values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`MacdState`],
/// or a [`TechalysisError`] error if the calculation fails.
pub fn macd_into(
    data: &[Float],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    output_macd: &mut [Float],
    output_signal: &mut [Float],
    output_histogram: &mut [Float],
) -> Result<MacdState, TechalysisError> {
    if fast_period >= slow_period {
        return Err(TechalysisError::BadParam(
            "Fast period must be less than slow period".to_string(),
        ));
    }

    if fast_period <= 1 || slow_period <= 1 || signal_period <= 1 {
        return Err(TechalysisError::BadParam(
            "Periods must be greater than 1".to_string(),
        ));
    }

    let skip_period = slow_period + signal_period;
    let slow_ema_start_idx = 0;
    let fast_ema_start_idx = slow_period - fast_period;
    let signal_start_idx = slow_period;
    let macd_start_idx = (slow_period - 1) + (signal_period - 1);

    let len: usize = data.len();

    if len < skip_period {
        return Err(TechalysisError::InsufficientData);
    }

    output_macd[..macd_start_idx].fill(Float::NAN);
    output_signal[..macd_start_idx].fill(Float::NAN);
    output_histogram[..macd_start_idx].fill(Float::NAN);

    let fast_alpha = period_to_alpha(fast_period, None)?;
    let slow_alpha = period_to_alpha(slow_period, None)?;
    let signal_alpha = period_to_alpha(signal_period, None)?;

    let mut fast_sum = 0.0;
    let mut slow_sum = 0.0;

    for (idx, value) in data
        .iter()
        .take(fast_ema_start_idx)
        .skip(slow_ema_start_idx)
        .enumerate()
    {
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {value:?}",
            )));
        }
        slow_sum += value;
    }

    for (idx, value) in data
        .iter()
        .take(signal_start_idx)
        .skip(fast_ema_start_idx)
        .enumerate()
    {
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {value:?}"
            )));
        }
        slow_sum += value;
        fast_sum += value;
    }
    let mut fast_ema = fast_sum / fast_period as Float;
    let mut slow_ema = slow_sum / slow_period as Float;
    let mut sum_macd = fast_ema - slow_ema;

    for (idx, value) in data
        .iter()
        .take(macd_start_idx)
        .skip(signal_start_idx)
        .enumerate()
    {
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {value:?}"
            )));
        }
        fast_ema = ema_next_unchecked(*value, fast_ema, fast_alpha);
        slow_ema = ema_next_unchecked(*value, slow_ema, slow_alpha);
        sum_macd += fast_ema - slow_ema;
    }

    fast_ema = ema_next_unchecked(data[macd_start_idx], fast_ema, fast_alpha);
    slow_ema = ema_next_unchecked(data[macd_start_idx], slow_ema, slow_alpha);
    output_macd[macd_start_idx] = fast_ema - slow_ema;
    sum_macd += output_macd[macd_start_idx];
    output_signal[macd_start_idx] = sum_macd / signal_period as Float;
    output_histogram[macd_start_idx] = output_macd[macd_start_idx] - output_signal[macd_start_idx];

    for idx in macd_start_idx + 1..len {
        let data = data[idx];
        if !data.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {data:?}"
            )));
        }
        (
            fast_ema,
            slow_ema,
            output_macd[idx],
            output_signal[idx],
            output_histogram[idx],
        ) = macd_next_unchecked(
            data,
            fast_ema,
            slow_ema,
            output_signal[idx - 1],
            fast_alpha,
            slow_alpha,
            signal_alpha,
        );
        if !output_macd[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output_macd[idx]));
        }
        if !output_signal[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output_signal[idx]));
        }
        if !output_histogram[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output_histogram[idx]));
        }
    }

    Ok(MacdState {
        macd: output_macd[len - 1],
        signal: output_signal[len - 1],
        histogram: output_histogram[len - 1],
        fast_ema,
        slow_ema,
        fast_period,
        slow_period,
        signal_period,
    })
}

#[inline(always)]
fn macd_next_unchecked(
    new_value: Float,
    prev_fast_ema: Float,
    prev_slow_ema: Float,
    prev_signal: Float,
    fast_alpha: Float,
    slow_alpha: Float,
    signal_alpha: Float,
) -> (Float, Float, Float, Float, Float) {
    let fast_ema = ema_next_unchecked(new_value, prev_fast_ema, fast_alpha);
    let slow_ema = ema_next_unchecked(new_value, prev_slow_ema, slow_alpha);
    let macd = fast_ema - slow_ema;
    let signal = ema_next_unchecked(macd, prev_signal, signal_alpha);
    let histogram = macd - signal;
    (fast_ema, slow_ema, macd, signal, histogram)
}
