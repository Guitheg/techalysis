use super::ema::period_to_alpha;
use crate::errors::TechnicalysisError;
use crate::indicators::ema::ema_next;
use crate::result::TechnicalysisResult;

#[derive(Debug)]
pub struct MacdResult {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
    pub histogram: Vec<f64>,
}

impl TechnicalysisResult for MacdResult {
    fn to_vec(self) -> Vec<Vec<f64>> {
        vec![self.macd, self.signal, self.histogram]
    }
}

pub fn macd(
    data_array: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<MacdResult, TechnicalysisError> {
    let size: usize = data_array.len();

    let mut output_macd = vec![f64::NAN; size];
    let mut output_signal = vec![f64::NAN; size];
    let mut output_histogram = vec![f64::NAN; size];

    core_macd(
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
    })
}

pub fn core_macd(
    data_array: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    output_macd: &mut [f64],
    output_signal: &mut [f64],
    output_histogram: &mut [f64],
) -> Result<(), TechnicalysisError> {
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
    let mut fast_ema_prev = fast_sum / fast_period as f64;
    let mut slow_ema_prev = slow_sum / slow_period as f64;
    let mut sum_macd = fast_ema_prev - slow_ema_prev;

    for value in data_array
        .iter()
        .take(macd_start_idx)
        .skip(signal_start_idx)
    {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        fast_ema_prev = ema_next(value, &fast_ema_prev, &fast_alpha);
        slow_ema_prev = ema_next(value, &slow_ema_prev, &slow_alpha);
        sum_macd += fast_ema_prev - slow_ema_prev;
    }

    fast_ema_prev = ema_next(&data_array[macd_start_idx], &fast_ema_prev, &fast_alpha);
    slow_ema_prev = ema_next(&data_array[macd_start_idx], &slow_ema_prev, &slow_alpha);
    output_macd[macd_start_idx] = fast_ema_prev - slow_ema_prev;
    sum_macd += output_macd[macd_start_idx];
    let mut signal_ema_prev = sum_macd / signal_period as f64;
    output_signal[macd_start_idx] = signal_ema_prev;
    output_histogram[macd_start_idx] = output_macd[macd_start_idx] - output_signal[macd_start_idx];

    // Main loop
    for idx in macd_start_idx + 1..size {
        let data = data_array[idx];
        if data.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        fast_ema_prev = ema_next(&data, &fast_ema_prev, &fast_alpha);
        slow_ema_prev = ema_next(&data, &slow_ema_prev, &slow_alpha);
        output_macd[idx] = fast_ema_prev - slow_ema_prev;
        signal_ema_prev = ema_next(&output_macd[idx], &signal_ema_prev, &signal_alpha);
        output_signal[idx] = signal_ema_prev;
        output_histogram[idx] = output_macd[idx] - output_signal[idx];
    }

    Ok(())
}
