use crate::errors::TechalysisError;
use crate::indicators::dema::{
    dema_next_unchecked, dema_skip_period_unchecked, init_dema_unchecked,
};
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};

use crate::types::Float;

#[derive(Debug)]
pub struct TemaResult {
    pub values: Vec<Float>,
    pub state: TemaState,
}

#[derive(Debug, Clone, Copy)]
pub struct TemaState {
    pub tema: Float,
    pub ema_1: Float,
    pub ema_2: Float,
    pub ema_3: Float,
    pub period: usize,
    pub alpha: Float,
}

impl From<TemaResult> for Vec<Float> {
    fn from(result: TemaResult) -> Self {
        result.values
    }
}

impl TemaState {
    pub fn next(&self, new_value: Float) -> Result<TemaState, TechalysisError> {
        tema_next(
            new_value,
            self.ema_1,
            self.ema_2,
            self.ema_3,
            self.period,
            Some(self.alpha),
        )
    }
}

pub fn tema_next(
    new_value: Float,
    prev_ema_1: Float,
    prev_ema_2: Float,
    prev_ema_3: Float,
    period: usize,
    alpha: Option<Float>,
) -> Result<TemaState, TechalysisError> {
    let alpha = match alpha {
        Some(a) => a,
        None => period_to_alpha(period, None)?,
    };
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "Period must be greater than 1".to_string(),
        ));
    }
    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}",
        )));
    }

    if !prev_ema_1.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_ema_1 = {prev_ema_1:?}",
        )));
    }
    if !prev_ema_2.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_ema_2 = {prev_ema_2:?}",
        )));
    }
    if !alpha.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!("alpha = {alpha:?}")));
    }

    let (ema_1, ema_2, ema_3, tema) =
        tema_next_unchecked(new_value, prev_ema_1, prev_ema_2, prev_ema_3, alpha);
    if !tema.is_finite() {
        return Err(TechalysisError::Overflow(0, tema));
    }
    Ok(TemaState {
        tema,
        ema_1,
        ema_2,
        ema_3,
        period,
        alpha,
    })
}

pub fn tema(
    data_array: &[Float],
    period: usize,
    alpha: Option<Float>,
) -> Result<TemaResult, TechalysisError> {
    let mut output = vec![0.0; data_array.len()];

    let tema_state = tema_into(data_array, period, alpha, &mut output)?;

    Ok(TemaResult {
        values: output,
        state: tema_state,
    })
}

pub fn tema_into(
    data: &[Float],
    period: usize,
    alpha: Option<Float>,
    output: &mut [Float],
) -> Result<TemaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / period as Float;
    let skip_period = tema_skip_period_unchecked(period);

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
    let (mut ema_1, mut ema_2, mut ema_3, output_value) =
        init_tema_unchecked(data, period, inv_period, skip_period, alpha, output)?;
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

        (ema_1, ema_2, ema_3, output[idx]) =
            tema_next_unchecked(data[idx], ema_1, ema_2, ema_3, alpha);

        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }

    Ok(TemaState {
        tema: output[len - 1],
        ema_1,
        ema_2,
        ema_3,
        period,
        alpha,
    })
}

#[inline(always)]
pub fn tema_next_unchecked(
    new_value: Float,
    prev_ema_1: Float,
    prev_ema_2: Float,
    prev_ema_3: Float,
    alpha: Float,
) -> (Float, Float, Float, Float) {
    let ema_1 = ema_next_unchecked(new_value, prev_ema_1, alpha);
    let ema_2 = ema_next_unchecked(ema_1, prev_ema_2, alpha);
    let ema_3 = ema_next_unchecked(ema_2, prev_ema_3, alpha);
    (ema_1, ema_2, ema_3, calculate_tema(ema_1, ema_2, ema_3))
}

#[inline(always)]
pub(crate) fn init_tema_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    skip_period: usize,
    alpha: Float,
    output: &mut [Float],
) -> Result<(Float, Float, Float, Float), TechalysisError> {
    let dema_skip_period = dema_skip_period_unchecked(period);
    let (mut ema_1, mut ema_2, _) =
        init_dema_unchecked(data, period, inv_period, dema_skip_period, alpha, output)?;
    output[dema_skip_period] = Float::NAN;

    let mut sum_ema_3 = ema_2;
    for idx in dema_skip_period + 1..skip_period {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        (ema_1, ema_2, _) = dema_next_unchecked(data[idx], ema_1, ema_2, alpha);
        sum_ema_3 += ema_2;
        output[idx] = Float::NAN;
    }
    (ema_1, ema_2, _) = dema_next_unchecked(data[skip_period], ema_1, ema_2, alpha);
    sum_ema_3 += ema_2;
    let ema_3 = sum_ema_3 * inv_period;

    Ok((ema_1, ema_2, ema_3, calculate_tema(ema_1, ema_2, ema_3)))
}

#[inline(always)]
fn calculate_tema(ema_1: Float, ema_2: Float, ema_3: Float) -> Float {
    (3.0 * ema_1) - (3.0 * ema_2) + ema_3
}

pub fn tema_skip_period_unchecked(period: usize) -> usize {
    3 * (period - 1)
}
