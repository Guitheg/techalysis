use crate::errors::TechnicalysisError;

const DEFAULT_SMOOTHING: f64 = 2.0;

pub fn period_to_alpha(period: usize, smoothing: Option<f64>) -> Result<f64, TechnicalysisError> {
    if period == 0 {
        return Err(TechnicalysisError::BadParam(
            "Period must be greater than 0".to_string(),
        ));
    }

    let smoothing = match smoothing {
        Some(s) => {
            if s <= 0.0 {
                return Err(TechnicalysisError::BadParam(
                    "Smoothing must be greater than 0".to_string(),
                ));
            }
            s
        }
        None => DEFAULT_SMOOTHING,
    };

    Ok(smoothing / (period as f64 + 1.0))
}

pub fn ema(
    data_array: &[f64],
    window_size: usize,
    alpha: Option<f64>,
) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();
    if window_size == 0 || size < window_size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if window_size == 1 {
        return Ok(data_array.to_vec());
    }

    let alpha = match alpha {
        Some(alpha) => alpha,
        None => period_to_alpha(window_size, None)?,
    };
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
        ema_prev = ema_next(&data_array[idx], &ema_prev, &alpha);
        result[idx] = ema_prev;
    }

    Ok(result)
}

#[inline(always)]
pub fn ema_next(new_value: &f64, prev_ema: &f64, alpha: &f64) -> f64 {
    new_value * alpha + prev_ema * (1.0 - alpha)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_period_to_alpha() {
        assert_eq!(period_to_alpha(10, None).unwrap(), 0.18181818181818182);
        assert_eq!(period_to_alpha(10, Some(2.0)).unwrap(), 0.18181818181818182);
        assert!(period_to_alpha(0, None).is_err());
        assert!(period_to_alpha(10, Some(0.0)).is_err());
    }
}
