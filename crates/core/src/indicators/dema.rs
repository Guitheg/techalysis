use crate::errors::TechalysisError;
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};
use crate::indicators::sma::init_sma_unchecked;
use crate::types::Float;

#[derive(Debug)]
pub struct DemaResult {
    pub values: Vec<Float>,
    pub state: DemaState,
}

#[derive(Debug, Clone, Copy)]
pub struct DemaState {
    pub dema: Float,
    pub ema_1: Float,
    pub ema_2: Float,
    pub period: usize,
    pub alpha: Float,
}

impl DemaState {
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

        if !self.ema_1.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema_1 = {:?}", self.ema_1
            )));
        }
        if !self.ema_2.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.ema_2 = {:?}", self.ema_2
            )));
        }
        if !self.alpha.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!("self.alpha = {:?}", self.alpha)));
        }

        let (dema, ema_1, ema_2) = dema_next_unchecked(
            new_value,
            self.ema_1, 
            self.ema_2,
            self.alpha
        );

        if !dema.is_finite() {
            return Err(TechalysisError::Overflow(0, dema));
        }
        self.dema = dema;
        self.ema_1 = ema_1;
        self.ema_2 = ema_2;
        Ok(())
    }
}

pub fn dema(
    data_array: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<DemaResult, TechalysisError> {
    let mut output = vec![0.0; data_array.len()];

    let dema_state = dema_into(data_array, period, alpha, &mut output)?;

    Ok(DemaResult {
        values: output,
        state: dema_state,
    })
}

pub fn dema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<DemaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    let skip_period = dema_skip_period_unchecked(period);

    if period == 0 || len < skip_period + 1 {
        return Err(TechalysisError::InsufficientData);
    }

    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "EMA period must be greater than 1".to_string(),
        ));
    }

    let alpha = match alpha {
        Some(alpha) => alpha,
        None => period_to_alpha(period, None)?,
    };
    let (output_value, mut ema_1, mut ema_2) =
        init_dema_unchecked(data, period, inv_period, skip_period, alpha, output)?;
    output[skip_period] = output_value;
    if !output[skip_period].is_finite() {
        return Err(TechalysisError::Overflow(skip_period, output[skip_period]));
    }

    for idx in skip_period + 1..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }

        (output[idx], ema_1, ema_2) = dema_next_unchecked(data[idx], ema_1, ema_2, alpha);

        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }

    Ok(DemaState {
        dema: output[len - 1],
        ema_1,
        ema_2,
        period,
        alpha,
    })
}

#[inline(always)]
pub(crate) fn dema_next_unchecked(
    new_value: Float,
    prev_ema_1: Float,
    prev_ema_2: Float,
    alpha: Float,
) -> (Float, Float, Float) {
    let ema_1 = ema_next_unchecked(new_value, prev_ema_1, alpha);
    let ema_2 = ema_next_unchecked(ema_1, prev_ema_2, alpha);
    (calculate_dema(ema_1, ema_2), ema_1, ema_2)
}

#[inline(always)]
pub(crate) fn init_dema_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    skip_period: usize,
    alpha: Float,
    output: &mut [Float],
) -> Result<(Float, Float, Float), TechalysisError> {
    let mut ema_1 = init_sma_unchecked(data, period, inv_period, output)?;

    let mut sum_ema_2 = ema_1;
    for idx in period..skip_period {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        ema_1 = ema_next_unchecked(data[idx], ema_1, alpha);
        sum_ema_2 += ema_1;
        output[idx] = Float::NAN;
    }
    ema_1 = ema_next_unchecked(data[skip_period], ema_1, alpha);
    sum_ema_2 += ema_1;
    let ema_2 = sum_ema_2 * inv_period;

    Ok((calculate_dema(ema_1, ema_2), ema_1, ema_2))
}

#[inline(always)]
fn calculate_dema(ema_1: Float, ema_2: Float) -> Float {
    (2.0 * ema_1) - ema_2
}

pub fn dema_skip_period_unchecked(period: usize) -> usize {
    2 * (period - 1)
}
