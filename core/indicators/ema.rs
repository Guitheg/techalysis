use crate::errors::TechnicalysisError;

pub fn ema(
    data_array: &[f64],
    window_size: usize,
    smoothing: f64,
) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();
    if window_size == 0 || size < window_size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if window_size == 1 {
        return Ok(data_array.to_vec());
    }

    if smoothing <= 0.0 {
        return Err(TechnicalysisError::BadParam(
            "Smoothing must be greater than 0".to_string(),
        ));
    }

    let alpha = smoothing / (window_size as f64 + 1.0);
    let alpha_c = 1.0 - alpha;
    let mut result = vec![f64::NAN; size];

    let mut sum = 0.0;
    for &value in &data_array[..window_size] {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        sum += value;
    }
    let mut ema_prev = sum / window_size as f64;
    result[window_size - 1] = ema_prev;

    for idx in window_size..size {
        if data_array[idx].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        ema_prev = data_array[idx] * alpha + ema_prev * alpha_c;
        result[idx] = ema_prev;
    }

    Ok(result)
}
