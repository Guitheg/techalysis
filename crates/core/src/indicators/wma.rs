use std::collections::VecDeque;

use crate::errors::TechalysisError;
use crate::types::Float;

#[derive(Debug)]
pub struct WmaResult {
    pub values: Vec<Float>,
    pub state: WmaState,
}

#[derive(Debug, Clone)]
pub struct WmaState {
    pub wma: Float,
    pub period: usize,
    pub period_sub: Float,
    pub period_sum: Float,
    pub window: VecDeque<Float>,
}

impl From<WmaResult> for Vec<f64> {
    fn from(result: WmaResult) -> Self {
        result.values
    }
}

impl WmaState {
    pub fn next(&self, new_value: Float) -> Result<WmaState, TechalysisError> {
        wma_next(
            new_value,
            self.wma,
            self.period,
            self.period_sub,
            self.period_sum,
            &self.window,
        )
    }
}

pub fn wma_next(
    new_value: Float,
    prev_wma: Float,
    period: usize,
    period_sub: Float,
    period_sum: Float,
    window: &VecDeque<Float>,
) -> Result<WmaState, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "WMA period must be greater than 1".to_string(),
        ));
    }
    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}"
        )));
    }
    if !prev_wma.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_wma = {prev_wma:?}"
        )));
    }
    if window.len() != period {
        return Err(TechalysisError::BadParam(
            "Window length must match the WMA period".to_string(),
        ));
    }

    for (idx, &value) in window.iter().enumerate() {
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "window[{idx}] = {value:?}"
            )));
        }
    }

    let mut window = window.clone();
    let inv_weight_sum = inv_weight_sum_linear(period);

    let old_value = window
        .pop_front()
        .ok_or(TechalysisError::InsufficientData)?;
    window.push_back(new_value);

    let (wma, new_period_sub, new_period_sum) = wma_next_unchecked(
        new_value,
        old_value,
        period as Float,
        period_sub,
        period_sum,
        inv_weight_sum,
    );

    Ok(WmaState {
        wma,
        period,
        period_sub: new_period_sub,
        period_sum: new_period_sum,
        window,
    })
}

pub fn wma(data: &[Float], period: usize) -> Result<WmaResult, TechalysisError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let wma_state = wma_into(data, period, &mut output)?;
    Ok(WmaResult {
        values: output,
        state: wma_state,
    })
}

pub fn wma_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<WmaState, TechalysisError> {
    let len = data.len();
    let inv_weight_sum = inv_weight_sum_linear(period);
    if period == 0 || period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "WMA period must be greater than 1".to_string(),
        ));
    }

    if output.len() < len {
        return Err(TechalysisError::BadParam(
            "Output array must be at least as long as the input data array".to_string(),
        ));
    }

    let (mut period_sub, mut period_sum) =
        init_wma_unchecked(data, period, inv_weight_sum, output)?;

    for idx in period..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        (output[idx], period_sub, period_sum) = wma_next_unchecked(
            data[idx],
            data[idx - period],
            period as Float,
            period_sub,
            period_sum,
            inv_weight_sum,
        );
    }
    Ok(WmaState {
        wma: output[len - 1],
        period,
        period_sub,
        period_sum,
        window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

#[inline(always)]
pub fn wma_next_unchecked(
    new_value: Float,
    old_value: Float,
    period: Float,
    period_sub: Float,
    period_sum: Float,
    inv_weight_sum: Float,
) -> (Float, Float, Float) {
    let new_period_sub = period_sub - old_value + new_value;
    let new_weighted_sum = period_sum + new_value * period;
    (
        new_weighted_sum * inv_weight_sum,
        new_period_sub,
        new_weighted_sum - new_period_sub,
    )
}

#[inline(always)]
pub(crate) fn init_wma_unchecked(
    data: &[Float],
    period: usize,
    inv_weight_sum: Float,
    output: &mut [Float],
) -> Result<(Float, Float), TechalysisError> {
    let mut period_sub: Float = 0.0;
    let mut period_sum: Float = 0.0;
    for idx in 0..period {
        let weight = idx as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        }
        period_sub += value;
        period_sum += value * weight;
        output[idx] = Float::NAN;
    }
    output[period - 1] = (period_sum + period_sub) * inv_weight_sum;
    Ok((period_sub, period_sum))
}

#[inline(always)]
fn inv_weight_sum_linear(period: usize) -> Float {
    2.0 / (period * (period + 1)) as Float
}
