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

#[cfg(test)]
mod tests {
    use crate::oracle_test;
    use crate::{errors::TechnicalysisError, features::ema::ema};

    oracle_test!(ema, |x: &[f64]| ema(x, 30, 2.0));

    #[test]
    fn test_ema_with_nan_must_fail() {
        let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = ema(&data, opt_in_time_period, 2f64);
        assert!(matches!(result, Err(TechnicalysisError::UnexpectedNan)))
    }
}
