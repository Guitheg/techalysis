use crate::helper::{errors::TechnicalysisError, loopback::lookback};

pub fn ema(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let lookback_result = lookback(window_size)?;
    if data_array.len() < window_size {
        return Err(TechnicalysisError::InsufficientData);
    }

    let start_idx = if lookback_result > 0 {
        lookback_result
    } else {
        0
    };

    let period_as_double = window_size as f64;
    let mut data_out = Vec::with_capacity(data_array.len());

    let mut trailing_idx = start_idx - lookback_result;
    let mut total_in_period = 0.0;
    let mut nan_counter = 0;

    if window_size > 1 {
        for data in data_array[trailing_idx..start_idx].iter() {
            if (*data).is_nan() {
                nan_counter = window_size;
                total_in_period = 0.0;
            } else {
                total_in_period += data;
            }

            if nan_counter != 0 {
                nan_counter = nan_counter.saturating_sub(1);
            }
            data_out.push(f64::NAN);
        }
    }

    Ok(data_out)
}
