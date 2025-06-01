use crate::errors::TechnicalysisError;
use std::f64;

#[inline(always)]
fn calculate_rsi(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_loss == 0.0 {
        if avg_gain == 0.0 {
            return 50.0;
        }
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}

pub fn rsi(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let size: usize = data_array.len();
    let mut output = vec![f64::NAN; size];
    core_rsi(data_array, window_size, &mut output)?;
    Ok(output)
}

pub fn core_rsi(
    data_array: &[f64],
    window_size: usize,
    output: &mut [f64],
) -> Result<(), TechnicalysisError> {
    let size = data_array.len();
    let period = window_size as f64;
    if window_size == 0 || window_size + 1 > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if window_size == 1 {
        return Err(TechnicalysisError::BadParam(
            "RSI window size must be greater than 1".to_string(),
        ));
    }

    let k = 1.0 / window_size as f64;
    let one_minus_k = 1.0 - k;

    let mut avg_gain: f64 = 0.0;
    let mut avg_loss: f64 = 0.0;
    for i in 1..=window_size {
        let delta = data_array[i] - data_array[i - 1];
        if delta.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        if delta > 0.0 {
            avg_gain += delta;
        } else {
            avg_loss -= delta;
        }
    }
    avg_gain /= period;
    avg_loss /= period;
    output[window_size] = calculate_rsi(avg_gain, avg_loss);

    for i in (window_size + 1)..size {
        let delta = data_array[i] - data_array[i - 1];
        if delta.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        if delta > 0.0 {
            avg_gain = avg_gain * one_minus_k + delta * k;
            avg_loss *= one_minus_k;
        } else if delta < 0.0 {
            avg_gain *= one_minus_k;
            avg_loss = avg_loss * one_minus_k - delta * k;
        } else {
            avg_gain *= one_minus_k;
            avg_loss *= one_minus_k;
        }
        output[i] = calculate_rsi(avg_gain, avg_loss);
    }
    Ok(())
}
