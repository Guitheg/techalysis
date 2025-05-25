use crate::errors::TechnicalysisError;
use std::f64;

pub fn sma(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();
    if window_size == 0 || window_size > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if window_size == 1 {
        return Ok(data_array.to_vec());
    }

    let mut result = vec![f64::NAN; size];

    let mut running_sum: f64 = 0.0;
    for &value in &data_array[..window_size] {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        } else {
            running_sum += value;
        }
    }
    result[window_size - 1] = running_sum / window_size as f64;

    for idx in window_size..size {
        if data_array[idx].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        running_sum += data_array[idx] - data_array[idx - window_size];
        result[idx] = running_sum / window_size as f64;
    }
    Ok(result)
}
