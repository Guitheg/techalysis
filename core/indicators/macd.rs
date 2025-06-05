use super::ema::period_to_alpha;
use crate::errors::TechnicalysisError;
use crate::indicators::ema::ema_next_unchecked;

#[derive(Debug)]
pub struct MacdResult {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
    pub histogram: Vec<f64>,
    pub state: MacdState,
}

impl From<MacdResult> for (Vec<f64>, Vec<f64>, Vec<f64>) {
    fn from(result: MacdResult) -> Self {
        (result.macd, result.signal, result.histogram)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MacdState {
    pub fast_ema: f64,
    pub slow_ema: f64,
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
    pub fast_period: usize,
    pub slow_period: usize,
    pub signal_period: usize,
}

impl MacdState {
    pub fn next(&self, new_value: f64) -> Result<MacdState, TechnicalysisError> {
        Ok(macd_next(
            new_value,
            self.fast_ema,
            self.slow_ema,
            self.signal,
            self.fast_period,
            self.slow_period,
            self.signal_period,
        )?)
    }
}

pub fn macd(
    data_array: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<MacdResult, TechnicalysisError> {
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
    data_array: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    output_macd: &mut [f64],
    output_signal: &mut [f64],
    output_histogram: &mut [f64],
) -> Result<MacdState, TechnicalysisError> {
    if fast_period >= slow_period {
        return Err(TechnicalysisError::BadParam(
            "Fast period must be less than slow period".to_string(),
        ));
    }

    // The calculation is not necessary and can be simplify but it's conceptually descriptive
    let skip_period = slow_period + signal_period;
    let slow_ema_start_idx = 0;
    let fast_ema_start_idx = slow_period - fast_period;
    let signal_start_idx = slow_period;
    let macd_start_idx = (slow_period - 1) + (signal_period - 1);

    let size: usize = data_array.len();

    if size < skip_period {
        return Err(TechnicalysisError::InsufficientData);
    }

    output_macd[..macd_start_idx].fill(f64::NAN);
    output_signal[..macd_start_idx].fill(f64::NAN);
    output_histogram[..macd_start_idx].fill(f64::NAN);

    let fast_alpha = period_to_alpha(fast_period, None)?;
    let slow_alpha = period_to_alpha(slow_period, None)?;
    let signal_alpha = period_to_alpha(signal_period, None)?;

    let mut fast_sum = 0.0;
    let mut slow_sum = 0.0;

    for value in data_array
        .iter()
        .take(fast_ema_start_idx)
        .skip(slow_ema_start_idx)
    {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        slow_sum += value;
    }

    for value in data_array
        .iter()
        .take(signal_start_idx)
        .skip(fast_ema_start_idx)
    {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        slow_sum += value;
        fast_sum += value;
    }
    let mut fast_ema = fast_sum / fast_period as f64;
    let mut slow_ema = slow_sum / slow_period as f64;
    let mut sum_macd = fast_ema - slow_ema;

    for value in data_array
        .iter()
        .take(macd_start_idx)
        .skip(signal_start_idx)
    {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        fast_ema = ema_next_unchecked(*value, fast_ema, fast_alpha);
        slow_ema = ema_next_unchecked(*value, slow_ema, slow_alpha);
        sum_macd += fast_ema - slow_ema;
    }

    fast_ema = ema_next_unchecked(data_array[macd_start_idx], fast_ema, fast_alpha);
    slow_ema = ema_next_unchecked(data_array[macd_start_idx], slow_ema, slow_alpha);
    output_macd[macd_start_idx] = fast_ema - slow_ema;
    sum_macd += output_macd[macd_start_idx];
    output_signal[macd_start_idx] = sum_macd / signal_period as f64;
    output_histogram[macd_start_idx] = output_macd[macd_start_idx] - output_signal[macd_start_idx];

    for idx in macd_start_idx + 1..size {
        let data = data_array[idx];
        if data.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
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
    }

    Ok(MacdState {
        fast_ema,
        slow_ema,
        macd: output_macd[size - 1],
        signal: output_signal[size - 1],
        histogram: output_histogram[size - 1],
        fast_period,
        slow_period,
        signal_period,
    })
}

pub fn macd_next(
    new_value: f64,
    prev_fast_ema: f64,
    prev_slow_ema: f64,
    prev_signal: f64,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<MacdState, TechnicalysisError> {
    if fast_period >= slow_period {
        return Err(TechnicalysisError::BadParam(
            "Fast period must be less than slow period".to_string(),
        ));
    }

    let fast_alpha = period_to_alpha(fast_period, None)?;
    let slow_alpha = period_to_alpha(slow_period, None)?;
    let signal_alpha = period_to_alpha(signal_period, None)?;

    if new_value.is_nan()
        || prev_fast_ema.is_nan()
        || prev_slow_ema.is_nan()
        || prev_signal.is_nan()
    {
        return Err(TechnicalysisError::UnexpectedNan);
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
    new_value: f64,
    prev_fast_ema: f64,
    prev_slow_ema: f64,
    prev_signal: f64,
    fast_alpha: f64,
    slow_alpha: f64,
    signal_alpha: f64,
) -> (f64, f64, f64, f64, f64) {
    let fast_ema = ema_next_unchecked(new_value, prev_fast_ema, fast_alpha);
    let slow_ema = ema_next_unchecked(new_value, prev_slow_ema, slow_alpha);
    let macd = fast_ema - slow_ema;
    let signal = ema_next_unchecked(macd, prev_signal, signal_alpha);
    let histogram = macd - signal;
    (fast_ema, slow_ema, macd, signal, histogram)
}
