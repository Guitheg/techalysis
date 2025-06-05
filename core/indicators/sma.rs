use crate::errors::TechnicalysisError;
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
    pub fn next(&self, new_value: f64) -> Result<SmaState, TechnicalysisError> {
        Ok(sma_next(new_value, self.sma, &self.window, self.period)?)
    }
}

pub fn sma(data_array: &[f64], period: usize) -> Result<SmaResult, TechnicalysisError> {
    let size = data_array.len();
    let mut output = vec![0.0; size];
    let sma_state = sma_into(data_array, period, &mut output)?;
    Ok(SmaResult {
        values: output,
        state: sma_state,
    })
}

pub fn sma_into(
    data_array: &[f64],
    period: usize,
    output: &mut [f64],
) -> Result<SmaState, TechnicalysisError> {
    let size = data_array.len();
    let period_as_f64 = period as f64;
    if period == 0 || period > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechnicalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    let mut running_sum: f64 = 0.0;
    for idx in 0..period {
        let value = &data_array[idx];
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        } else {
            running_sum += value;
        }
        output[idx] = f64::NAN;
    }
    output[period - 1] = running_sum / period_as_f64;

    for idx in period..size {
        if data_array[idx].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        output[idx] = sma_next_unchecked(
            data_array[idx],
            data_array[idx - period],
            output[idx - 1],
            period_as_f64,
        );
    }
    Ok(SmaState {
        sma: output[size - 1],
        period,
        window: VecDeque::from(data_array[size - period..size].to_vec()),
    })
}

pub fn sma_next(
    new_value: f64,
    prev_sma: f64,
    window: &VecDeque<f64>,
    period: usize,
) -> Result<SmaState, TechnicalysisError> {
    if period < 1 {
        return Err(TechnicalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if new_value.is_nan() || prev_sma.is_nan() {
        return Err(TechnicalysisError::UnexpectedNan);
    }

    if window.len() != period {
        return Err(TechnicalysisError::BadParam(
            "Window length must match the SMA period".to_string(),
        ));
    }

    for &value in window {
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
    }

    let mut window = window.clone();

    let old_value = window
        .pop_front()
        .ok_or(TechnicalysisError::InsufficientData)?;
    window.push_back(new_value);

    Ok(SmaState {
        sma: sma_next_unchecked(new_value, old_value, prev_sma, period as f64),
        period,
        window: window,
    })
}

#[inline(always)]
pub fn sma_next_unchecked(new_value: f64, old_value: f64, prev_sma: f64, period: f64) -> f64 {
    prev_sma + (new_value - old_value) / period
}
