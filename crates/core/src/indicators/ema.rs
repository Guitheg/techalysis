use crate::errors::TechalysisError;

const DEFAULT_SMOOTHING: f64 = 2.0;

#[derive(Debug)]
pub struct EmaResult {
    pub values: Vec<f64>,
    pub state: EmaState,
}

impl From<EmaResult> for Vec<f64> {
    fn from(result: EmaResult) -> Self {
        result.values
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmaState {
    pub ema: f64,
    pub period: usize,
    pub alpha: f64,
}

impl EmaState {
    pub fn next(&self, new_value: f64) -> Result<EmaState, TechalysisError> {
        ema_next(new_value, self.ema, self.period, Some(self.alpha))
    }
}

pub fn period_to_alpha(period: usize, smoothing: Option<f64>) -> Result<f64, TechalysisError> {
    if period == 0 {
        return Err(TechalysisError::BadParam(
            "Period must be greater than 0".to_string(),
        ));
    }

    let smoothing = match smoothing {
        Some(s) => {
            if s <= 0.0 {
                return Err(TechalysisError::BadParam(
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
) -> Result<EmaResult, TechalysisError> {
    let mut output = vec![0.0; data_array.len()];
    let ema_state = ema_into(data_array, period, alpha, &mut output)?;
    Ok(EmaResult {
        values: output,
        state: ema_state,
    })
}

pub fn ema_into(
    data_array: &[f64],
    period: usize,
    alpha: Option<f64>,
    output: &mut [f64],
) -> Result<EmaState, TechalysisError> {
    let size = data_array.len();
    if period == 0 || size < period {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "EMA period must be greater than 1".to_string(),
        ));
    }

    let alpha = match alpha {
        Some(alpha) => alpha,
        None => period_to_alpha(period, None)?,
    };

    let mut sum = 0.0;
    for idx in 0..period {
        let value = &data_array[idx];
        if value.is_nan() {
            return Err(TechalysisError::UnexpectedNan);
        }
        sum += value;
        output[idx] = f64::NAN;
    }
    let mut ema_prev = sum / period as f64;
    output[period - 1] = ema_prev;

    for idx in period..size {
        if data_array[idx].is_nan() {
            return Err(TechalysisError::UnexpectedNan);
        }
        ema_prev = ema_next_unchecked(data_array[idx], ema_prev, alpha);
        output[idx] = ema_prev;
    }

    Ok(EmaState {
        ema: ema_prev,
        period,
        alpha,
    })
}

pub fn ema_next(
    new_value: f64,
    prev_ema: f64,
    period: usize,
    alpha: Option<f64>,
) -> Result<EmaState, TechalysisError> {
    let alpha = match alpha {
        Some(alpha) => alpha,
        None => period_to_alpha(period, None)?,
    };
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "Period must be greater than 1".to_string(),
        ));
    }

    if new_value.is_nan() || prev_ema.is_nan() || alpha.is_nan() {
        return Err(TechalysisError::UnexpectedNan);
    }

    let ema = ema_next_unchecked(new_value, prev_ema, alpha);
    Ok(EmaState { ema, period, alpha })
}

#[inline(always)]
pub fn ema_next_unchecked(new_value: f64, prev_ema: f64, alpha: f64) -> f64 {
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
