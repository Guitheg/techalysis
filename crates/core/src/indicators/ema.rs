use crate::errors::TechalysisError;
use crate::indicators::sma::init_sma_unchecked;
use crate::types::Float;

const DEFAULT_SMOOTHING: Float = 2.0;

#[derive(Debug)]
pub struct EmaResult {
    pub values: Vec<Float>,
    pub state: EmaState,
}

#[derive(Debug, Clone, Copy)]
pub struct EmaState {
    pub ema: Float,
    pub period: usize,
    pub alpha: Float,
}

impl EmaState {
    pub fn next(&mut self, new_value: Float) -> Result<(), TechalysisError> {
        if self.period <= 1 {
            return Err(TechalysisError::BadParam(
                "Period must be greater than 1".to_string(),
            ));
        }

        if !new_value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "new_value = {new_value:?}",
            )));
        }

        if !self.ema.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema = {:?}", self.ema
            )));
        }

        if !self.alpha.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!("alpha = {:?}", self.alpha)));
        }

        let ema = ema_next_unchecked(new_value, self.ema, self.alpha);
        if !ema.is_finite() {
            return Err(TechalysisError::Overflow(0, ema));
        }
        self.ema = ema;
        Ok(())
    }
}

pub fn ema(
    data_array: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<EmaResult, TechalysisError> {
    let mut output = vec![0.0; data_array.len()];
    let ema_state = ema_into(data_array, period, alpha, &mut output)?;
    Ok(EmaResult {
        values: output,
        state: ema_state,
    })
}

pub fn ema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<EmaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    if period == 0 || len < period {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "EMA period must be greater than 1".to_string(),
        ));
    }

    let alpha = get_alpha_value(alpha, period)?;

    output[period - 1] = init_sma_unchecked(data, period, inv_period, output)?;
    if !output[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(period - 1, output[period - 1]));
    }

    for idx in period..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        output[idx] = ema_next_unchecked(data[idx], output[idx - 1], alpha);
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }

    Ok(EmaState {
        ema: output[len - 1],
        period,
        alpha,
    })
}

#[inline(always)]
pub(crate) fn ema_next_unchecked(new_value: Float, prev_ema: Float, alpha: Float) -> Float {
    new_value * alpha + prev_ema * (1.0 - alpha)
}

pub fn period_to_alpha(period: usize, smoothing: Option<Float>) -> Result<Float, TechalysisError> {
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

    Ok(smoothing / (period as Float + 1.0))
}

pub(crate) fn get_alpha_value(alpha: Option<Float>, period: usize) -> Result<Float, TechalysisError> {
    match alpha {
        Some(a) => Ok(a),
        None => period_to_alpha(period, None),
    }
}