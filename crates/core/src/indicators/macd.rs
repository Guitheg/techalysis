use super::ema::period_to_alpha;
use crate::errors::TechalysisError;
use crate::indicators::ema::ema_next_unchecked;
use crate::types::Float;

#[derive(Debug)]
pub struct MacdResult {
    pub macd: Vec<Float>,
    pub signal: Vec<Float>,
    pub histogram: Vec<Float>,
    pub state: MacdState,
}

impl From<MacdResult> for (Vec<Float>, Vec<Float>, Vec<Float>) {
    fn from(result: MacdResult) -> Self {
        (result.macd, result.signal, result.histogram)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MacdState {
    pub fast_ema: Float,
    pub slow_ema: Float,
    pub macd: Float,
    pub signal: Float,
    pub histogram: Float,
    pub fast_period: usize,
    pub slow_period: usize,
    pub signal_period: usize,
}

impl MacdState {
    pub fn next(&self, new_value: Float) -> Result<MacdState, TechalysisError> {
        macd_next(
            new_value,
            self.fast_ema,
            self.slow_ema,
            self.signal,
            self.fast_period,
            self.slow_period,
            self.signal_period,
        )
    }
}

pub fn macd(
    data_array: &[Float],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<MacdResult, TechalysisError> {
    let size: usize = data_array.len();

    let mut output_macd = vec![0.0; size];
    let mut output_signal = vec![0.0; size];
    let mut output_histogram = vec![0.0; size];

    let macd_state = macd_into(
        data_array,
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

    // The calculation is not necessary and can be simplify but it's conceptually descriptive
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
        fast_ema,
        slow_ema,
        macd: output_macd[len - 1],
        signal: output_signal[len - 1],
        histogram: output_histogram[len - 1],
        fast_period,
        slow_period,
        signal_period,
    })
}

pub fn macd_next(
    new_value: Float,
    prev_fast_ema: Float,
    prev_slow_ema: Float,
    prev_signal: Float,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<MacdState, TechalysisError> {
    if fast_period >= slow_period {
        return Err(TechalysisError::BadParam(
            "Fast period must be less than slow period".to_string(),
        ));
    }

    let fast_alpha = period_to_alpha(fast_period, None)?;
    let slow_alpha = period_to_alpha(slow_period, None)?;
    let signal_alpha = period_to_alpha(signal_period, None)?;

    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}",
        )));
    }
    if !prev_fast_ema.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_fast_ema = {prev_fast_ema:?}",
        )));
    }
    if !prev_slow_ema.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_slow_ema = {prev_slow_ema:?}",
        )));
    }
    if !prev_signal.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_signal = {prev_signal:?}",
        )));
    }
    if fast_period <= 1 || slow_period <= 1 || signal_period <= 1 {
        return Err(TechalysisError::BadParam(
            "Periods must be greater than 1".to_string(),
        ));
    }

    let (fast_ema, slow_ema, macd, signal, histogram) = macd_next_unchecked(
        new_value,
        prev_fast_ema,
        prev_slow_ema,
        prev_signal,
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

    Ok(MacdState {
        fast_ema,
        slow_ema,
        macd,
        signal,
        histogram,
        fast_period,
        slow_period,
        signal_period,
    })
}

#[inline(always)]
pub fn macd_next_unchecked(
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
