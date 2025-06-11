use crate::errors::TechalysisError;
use crate::traits::State;
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
    pub last_window: VecDeque<Float>,
    pub period: usize,
}


impl State<Float> for SmaState {
    fn update(&mut self, sample: Float) -> Result<(), TechalysisError> {
        if self.period <= 1 {
            return Err(TechalysisError::BadParam(
                "SMA period must be greater than 1".to_string(),
            ));
        }
        if !sample.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "sample = {sample:?}"
            )));
        }
        if !self.sma.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.sma = {:?}", self.sma
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalysisError::BadParam(
                format!(
                    "SMA state last_window length ({}) does not match period ({})",
                    self.last_window.len(),
                    self.period
                )
            ));
        }

        for (idx, &value) in self.last_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalysisError::DataNonFinite(format!(
                    "window[{idx}] = {value:?}"
                )));
            }
        }

        let mut window = self.last_window.clone();

        let old_value = window
            .pop_front()
            .ok_or(TechalysisError::InsufficientData)?;
        window.push_back(sample);

        let sma = sma_next_unchecked(
            sample,
            old_value,
            self.sma,
            1.0 / (self.period as Float)
        );
        if !sma.is_finite() {
            return Err(TechalysisError::Overflow(0, sma));
        }
        self.sma = sma;
        self.last_window = window;
        
        Ok(())
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
        output[idx] =
            sma_next_unchecked(data[idx], data[idx - period], output[idx - 1], inv_period);
        if !output[idx].is_finite() {
            return Err(TechalysisError::Overflow(idx, output[idx]));
        }
    }
    Ok(SmaState {
        sma: output[len - 1],
        period,
        last_window: VecDeque::from(data[len - period..len].to_vec()),
    })
}

#[inline(always)]
pub(crate) fn sma_next_unchecked(
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
    Ok(sum * inv_period)
}
