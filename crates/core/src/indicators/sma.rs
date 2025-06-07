use crate::errors::TechalysisError;
use std::{collections::VecDeque, f64};

#[derive(Debug)]
pub struct SmaResult {
    pub values: Vec<f64>,
    pub state: SmaState,
}

impl From<SmaResult> for Vec<f64> {
    fn from(result: SmaResult) -> Self {
        result.values
    }
}

#[derive(Debug, Clone)]
pub struct SmaState {
    pub sma: f64,
    pub period: usize,
    pub window: VecDeque<f64>,
}

impl SmaState {
    pub fn next(&self, new_value: f64) -> Result<SmaState, TechalysisError> {
        sma_next(new_value, self.sma, &self.window, self.period)
    }
}

pub fn sma(data_array: &[f64], period: usize) -> Result<SmaResult, TechalysisError> {
    let size = data_array.len();
    let mut output = vec![0.0; size];
    let sma_state = sma_into(data_array, period, &mut output)?;
    Ok(SmaResult {
        values: output,
        state: sma_state,
    })
}

pub fn sma_into(
    data: &[f64],
    period: usize,
    output: &mut [f64],
) -> Result<SmaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / (period as f64);
    if period == 0 || period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if output.len() < len {
        return Err(TechalysisError::BadParam(
            "Output array must be at least as long as the input data array".to_string(),
        ));
    }

    init_sma_unchecked(data, period, inv_period, output)?;

    for idx in period..len {
        if !data[idx].is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                data[idx]
            )));
        }
        output[idx] =
            sma_next_unchecked(data[idx], data[idx - period], output[idx - 1], inv_period);
    }
    Ok(SmaState {
        sma: output[len - 1],
        period,
        window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

pub fn sma_next(
    new_value: f64,
    prev_sma: f64,
    window: &VecDeque<f64>,
    period: usize,
) -> Result<SmaState, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }
    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}"
        )));
    }
    if !prev_sma.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_sma = {prev_sma:?}"
        )));
    }
    if window.len() != period {
        return Err(TechalysisError::BadParam(
            "Window length must match the SMA period".to_string(),
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

    let old_value = window
        .pop_front()
        .ok_or(TechalysisError::InsufficientData)?;
    window.push_back(new_value);

    Ok(SmaState {
        sma: sma_next_unchecked(new_value, old_value, prev_sma, 1.0 / (period as f64)),
        period,
        window,
    })
}

#[inline(always)]
pub fn sma_next_unchecked(new_value: f64, old_value: f64, prev_sma: f64, inv_period: f64) -> f64 {
    prev_sma + (new_value - old_value) * inv_period
}

#[inline(always)]
pub(crate) fn init_sma_unchecked(
    data_array: &[f64],
    period: usize,
    inv_period: f64,
    output: &mut [f64],
) -> Result<f64, TechalysisError> {
    let mut sum: f64 = 0.0;
    for idx in 0..period {
        let value = &data_array[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        } else {
            sum += value;
        }
        output[idx] = f64::NAN;
    }
    output[period - 1] = sum * inv_period;
    Ok(sum)
}
