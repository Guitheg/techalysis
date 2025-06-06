use crate::errors::TechnicalysisError;
use std::{f64, vec};

#[derive(Debug)]
pub struct RsiResult {
    pub values: Vec<f64>,
    pub state: RsiState,
}

impl From<RsiResult> for Vec<f64> {
    fn from(result: RsiResult) -> Self {
        result.values
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RsiState {
    pub rsi: f64,
    pub prev_value: f64,
    pub avg_gain: f64,
    pub avg_loss: f64,
    pub period: usize,
}

impl RsiState {
    pub fn next(&self, new_value: f64) -> Result<RsiState, TechnicalysisError> {
        rsi_next(
            new_value,
            self.prev_value,
            self.avg_gain,
            self.avg_loss,
            self.period,
        )
    }
}

#[inline(always)]
fn calculate_rsi(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_loss == 0.0 {
        if avg_gain == 0.0 {
            return 50.0;
        }
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}

pub fn rsi(data_array: &[f64], window_size: usize) -> Result<RsiResult, TechnicalysisError> {
    let size: usize = data_array.len();
    let mut output = vec![0.0; size];
    let rsi_state = rsi_into(data_array, window_size, output.as_mut_slice())?;
    Ok(RsiResult {
        values: output,
        state: rsi_state,
    })
}

pub fn rsi_into(
    data_array: &[f64],
    period: usize,
    output_rsi: &mut [f64],
) -> Result<RsiState, TechnicalysisError> {
    let size = data_array.len();
    let period_as_float = period as f64;
    if period == 0 || period + 1 > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechnicalysisError::BadParam(
            "RSI window size must be greater than 1".to_string(),
        ));
    }

    if output_rsi.len() != size {
        return Err(TechnicalysisError::BadParam(
            "Output RSI length must match input data length".to_string(),
        ));
    }

    let mut avg_gain: f64 = 0.0;
    let mut avg_loss: f64 = 0.0;
    output_rsi[0] = f64::NAN;
    for i in 1..=period {
        let delta = data_array[i] - data_array[i - 1];
        if delta.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        if delta > 0.0 {
            avg_gain += delta;
        } else {
            avg_loss -= delta;
        }
        output_rsi[i] = f64::NAN;
    }
    avg_gain /= period_as_float;
    avg_loss /= period_as_float;
    output_rsi[period] = calculate_rsi(avg_gain, avg_loss);

    for i in (period + 1)..size {
        let delta = data_array[i] - data_array[i - 1];
        if delta.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        (output_rsi[i], avg_gain, avg_loss) = rsi_next_unchecked(
            data_array[i] - data_array[i - 1],
            avg_gain,
            avg_loss,
            period_as_float,
        );
    }
    Ok(RsiState {
        rsi: output_rsi[size - 1],
        prev_value: data_array[size - 1],
        avg_gain,
        avg_loss,
        period,
    })
}

pub fn rsi_next(
    new_value: f64,
    prev_value: f64,
    prev_avg_gain: f64,
    prev_avg_loss: f64,
    period: usize,
) -> Result<RsiState, TechnicalysisError> {
    if period <= 1 {
        return Err(TechnicalysisError::BadParam(
            "RSI period must be greater than 1".to_string(),
        ));
    }

    if new_value.is_nan() || prev_value.is_nan() || prev_avg_gain.is_nan() || prev_avg_loss.is_nan()
    {
        return Err(TechnicalysisError::UnexpectedNan);
    }

    let (rsi, avg_gain, avg_loss) = rsi_next_unchecked(
        new_value - prev_value,
        prev_avg_gain,
        prev_avg_loss,
        period as f64,
    );
    Ok(RsiState {
        rsi,
        prev_value: new_value,
        avg_gain,
        avg_loss,
        period,
    })
}

#[inline(always)]
pub fn rsi_next_unchecked(
    delta: f64,
    prev_avg_gain: f64,
    prev_avg_loss: f64,
    period: f64,
) -> (f64, f64, f64) {
    let k = 1.0 / period;
    let one_minus_k = 1.0 - k;
    let (avg_gain, avg_loss) = if delta > 0.0 {
        (
            prev_avg_gain * one_minus_k + delta * k,
            prev_avg_loss * one_minus_k,
        )
    } else if delta < 0.0 {
        (
            prev_avg_gain * one_minus_k,
            prev_avg_loss * one_minus_k - delta * k,
        )
    } else {
        (prev_avg_gain * one_minus_k, prev_avg_loss * one_minus_k)
    };

    (calculate_rsi(avg_gain, avg_loss), avg_gain, avg_loss)
}
