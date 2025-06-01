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
    period: usize,
    alpha: Option<f64>,
) -> Result<Vec<f64>, TechnicalysisError> {
    let mut output = vec![f64::NAN; data_array.len()];
    core_ema(data_array, period, alpha, &mut output)?;
    Ok(output)
}

pub fn core_ema(
    data_array: &[f64],
    period: usize,
    alpha: Option<f64>,
    output: &mut [f64],
) -> Result<(), TechnicalysisError> {
    let size = data_array.len();
    if period == 0 || size < period {
        return Err(TechnicalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechnicalysisError::BadParam(
            "EMA period must be greater than 1".to_string(),
        ));
    }

    let alpha = match alpha {
        Some(alpha) => alpha,
        None => period_to_alpha(period, None)?,
    };

    output[..period - 1].fill(f64::NAN);

    let mut sum = 0.0;
    for &value in &data_array[..period] {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        sum += value;
    }
    let mut ema_prev = sum / period as f64;
    output[period - 1] = ema_prev;

    for idx in period..size {
        if data_array[idx].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        ema_prev = ema_next(&data_array[idx], &ema_prev, &alpha);
        output[idx] = ema_prev;
    }

    Ok(())
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
