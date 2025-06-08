use crate::errors::TechalysisError;
use crate::types::Float;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct SmaResult {
    pub values: Vec<Float>,
    pub state: SmaState,
}

#[derive(Debug, Clone)]
pub struct SmaState {
    pub sma: Float,
    pub period: usize,
    pub window: VecDeque<Float>,
}

impl From<SmaResult> for Vec<Float> {
    fn from(result: SmaResult) -> Self {
        result.values
    }
}

impl SmaState {
    pub fn next(&self, new_value: Float) -> Result<SmaState, TechalysisError> {
        sma_next(new_value, self.sma, &self.window, self.period)
    }
}

pub fn sma(data: &[Float], period: usize) -> Result<SmaResult, TechalysisError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let sma_state = sma_into(data, period, &mut output)?;
    Ok(SmaResult {
        values: output,
        state: sma_state,
    })
}

pub fn sma_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<SmaState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / (period as Float);
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
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }
    Ok(SmaState {
        sma: output[len - 1],
        period,
        window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

pub fn sma_next(
    new_value: Float,
    prev_sma: Float,
    window: &VecDeque<Float>,
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
    let sma = sma_next_unchecked(new_value, old_value, prev_sma, 1.0 / (period as Float));
    if !sma.is_finite() {
        return Err(TechalysisError::Overflow(0, sma));
    }
    Ok(SmaState {
        sma,
        period,
        window,
    })
}

#[inline(always)]
pub fn sma_next_unchecked(
    new_value: Float,
    old_value: Float,
    prev_sma: Float,
    inv_period: Float,
) -> Float {
    prev_sma + (new_value - old_value) * inv_period
}

#[inline(always)]
pub(crate) fn init_sma_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    output: &mut [Float],
) -> Result<Float, TechalysisError> {
    let mut sum: Float = 0.0;
    for idx in 0..period {
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        } else {
            sum += value;
        }
        output[idx] = Float::NAN;
    }
    output[period - 1] = sum * inv_period;
    if !output[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(period - 1, output[period - 1]));
    }
    Ok(sum)
}
