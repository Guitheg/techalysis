use crate::errors::TechnicalysisError;
use std::f64;

fn calculate_rsi(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_loss == 0.0 {
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}

pub fn rsi(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();
    if window_size == 0 || window_size > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if window_size == 1 {
        return Ok(data_array.to_vec());
    }

    let mut result = vec![f64::NAN; size];

    let mut avg_gain: f64 = 0.0;
    let mut avg_loss: f64 = 0.0;
    for i in 1..=window_size {
        if data_array[i].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        let delta = data_array[i] - data_array[i - 1];
        if delta > 0.0 {
            avg_gain += delta;
        } else {
            avg_loss -= delta;
        }
    }
    avg_gain /= window_size as f64;
    avg_loss /= window_size as f64;
    result[window_size] = calculate_rsi(avg_gain, avg_loss);

    for i in (window_size + 1)..size {
        if data_array[i].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        let delta = data_array[i] - data_array[i - 1];
        if delta > 0.0 {
            avg_gain = (avg_gain * (window_size as f64 - 1.0) + delta) / window_size as f64;
            avg_loss = (avg_loss * (window_size as f64 - 1.0)) / window_size as f64;
        } else {
            avg_gain = (avg_gain * (window_size as f64 - 1.0)) / window_size as f64;
            avg_loss = (avg_loss * (window_size as f64 - 1.0) - delta) / window_size as f64;
        }
        result[i] = calculate_rsi(avg_gain, avg_loss);
    }
    Ok(result)
}
