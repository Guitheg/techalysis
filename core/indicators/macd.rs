use super::ema::period_to_alpha;
use crate::errors::TechnicalysisError;
use crate::indicators::step::ema_next;
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

    let mut macd = vec![f64::NAN; data_array.len()];
    let mut signal = vec![f64::NAN; data_array.len()];
    let mut histogram = vec![f64::NAN; data_array.len()];

    if size < skip_period {
        return Err(TechnicalysisError::InsufficientData);
    }

    let fast_alpha = period_to_alpha(fast_period, None)?;
    let slow_alpha = period_to_alpha(slow_period, None)?;
    let signal_alpha = period_to_alpha(signal_period, None)?;

    let mut fast_sum = 0.0;
    let mut slow_sum = 0.0;

    for idx in slow_ema_start_idx..fast_ema_start_idx {
        let value = data_array[idx];
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        slow_sum += value;
    }

    for idx in fast_ema_start_idx..signal_start_idx {
        let value = data_array[idx];
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        slow_sum += value;
        fast_sum += value;
    }
    let mut fast_ema_prev = fast_sum / fast_period as f64;
    let mut slow_ema_prev = slow_sum / slow_period as f64;
    let mut sum_macd = fast_ema_prev - slow_ema_prev;

    for idx in signal_start_idx..macd_start_idx {
        let value = data_array[idx];
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        fast_ema_prev = ema_next(&value, &fast_ema_prev, &fast_alpha);
        slow_ema_prev = ema_next(&value, &slow_ema_prev, &slow_alpha);
        sum_macd += fast_ema_prev - slow_ema_prev;
    }

    fast_ema_prev = ema_next(&data_array[macd_start_idx], &fast_ema_prev, &fast_alpha);
    slow_ema_prev = ema_next(&data_array[macd_start_idx], &slow_ema_prev, &slow_alpha);
    macd[macd_start_idx] = fast_ema_prev - slow_ema_prev;
    sum_macd += macd[macd_start_idx];
    let mut signal_ema_prev = sum_macd / signal_period as f64;
    signal[macd_start_idx] = signal_ema_prev;
    histogram[macd_start_idx] = macd[macd_start_idx] - signal[macd_start_idx];

    // Main loop
    for idx in macd_start_idx + 1..size {
        let data = data_array[idx];
        if data.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        fast_ema_prev = ema_next(&data, &fast_ema_prev, &fast_alpha);
        slow_ema_prev = ema_next(&data, &slow_ema_prev, &slow_alpha);
        macd[idx] = fast_ema_prev - slow_ema_prev;
        signal_ema_prev = ema_next(&macd[idx], &signal_ema_prev, &signal_alpha);
        signal[idx] = signal_ema_prev;
        histogram[idx] = macd[idx] - signal[idx];
    }

    Ok(MacdResult {
        macd,
        signal,
        histogram,
    })
}
