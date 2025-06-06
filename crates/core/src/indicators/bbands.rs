use std::collections::VecDeque;

use crate::errors::TechalysisError;
use crate::indicators::sma::sma_next_unchecked;

#[derive(Debug)]
pub struct BBandsResult {
    pub upper_band: Vec<f64>,
    pub middle_band: Vec<f64>,
    pub lower_band: Vec<f64>,
    pub state: BBandsState,
}

impl From<BBandsResult> for (Vec<f64>, Vec<f64>, Vec<f64>) {
    fn from(result: BBandsResult) -> Self {
        (result.upper_band, result.middle_band, result.lower_band)
    }
}

#[derive(Debug, Clone)]
pub struct BBandsState {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub sum_sq: f64,
    pub window: VecDeque<f64>,
    pub period: usize,
    pub std_up: f64,
    pub std_down: f64,
}

impl BBandsState {
    pub fn next(&self, new_value: f64) -> Result<BBandsState, TechalysisError> {
        bbands_next(
            new_value,
            self.middle,
            self.sum_sq,
            &self.window,
            self.period,
            self.std_up,
            self.std_down,
        )
    }
}

pub fn bbands(
    data_array: &[f64],
    period: usize,
    std_up: f64,
    std_down: f64,
) -> Result<BBandsResult, TechalysisError> {
    let mut output_upper = vec![0.0; data_array.len()];
    let mut output_middle = vec![0.0; data_array.len()];
    let mut output_lower = vec![0.0; data_array.len()];

    let bbands_state = bbands_into(
        data_array,
        period,
        std_up,
        std_down,
        &mut output_upper,
        &mut output_middle,
        &mut output_lower,
    )?;

    Ok(BBandsResult {
        upper_band: output_upper,
        middle_band: output_middle,
        lower_band: output_lower,
        state: bbands_state,
    })
}

pub fn bbands_into(
    data_array: &[f64],
    period: usize,
    std_up: f64,
    std_down: f64,
    output_upper: &mut [f64],
    output_middle: &mut [f64],
    output_lower: &mut [f64],
) -> Result<BBandsState, TechalysisError> {
    let len = data_array.len();
    let period_as_f64 = period as f64;
    if period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if std_up <= 0.0 || std_down <= 0.0 {
        return Err(TechalysisError::BadParam(
            "Standard deviations must be greater than 0".to_string(),
        ));
    }

    if output_upper.len() != len || output_middle.len() != len || output_lower.len() != len {
        return Err(TechalysisError::BadParam(
            "Output arrays must have the same length as input data".to_string(),
        ));
    }

    let mut running_sum: f64 = 0.0;
    let mut running_sum_sq: f64 = 0.0;
    for idx in 0..period {
        let value = &data_array[idx];
        if value.is_nan() {
            return Err(TechalysisError::UnexpectedNan);
        } else {
            running_sum += value;
            running_sum_sq += value * value;
        }
        output_upper[idx] = f64::NAN;
        output_middle[idx] = f64::NAN;
        output_lower[idx] = f64::NAN;
    }
    output_middle[period - 1] = running_sum / period_as_f64;
    output_upper[period - 1] = to_std_up(
        output_middle[period - 1],
        running_sum_sq,
        period_as_f64,
        std_up,
    );
    output_lower[period - 1] = to_std_down(
        output_middle[period - 1],
        running_sum_sq,
        period_as_f64,
        std_down,
    );

    for idx in period..len {
        if data_array[idx].is_nan() {
            return Err(TechalysisError::UnexpectedNan);
        }
        (
            output_upper[idx],
            output_middle[idx],
            output_lower[idx],
            running_sum_sq,
        ) = bbands_next_unchecked(
            data_array[idx],
            data_array[idx - period],
            output_middle[idx - 1],
            running_sum_sq,
            period_as_f64,
            std_up,
            std_down,
        );
    }

    Ok(BBandsState {
        upper: output_upper[len - 1],
        middle: output_middle[len - 1],
        lower: output_lower[len - 1],
        sum_sq: running_sum_sq,
        window: VecDeque::from(data_array[len - period..len].to_vec()),
        period,
        std_up,
        std_down,
    })
}

pub fn bbands_next(
    new_value: f64,
    prev_mean: f64,
    prev_sum_sq: f64,
    window: &VecDeque<f64>,
    period: usize,
    std_up: f64,
    std_down: f64,
) -> Result<BBandsState, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if new_value.is_nan() || prev_mean.is_nan() || prev_sum_sq.is_nan() {
        return Err(TechalysisError::UnexpectedNan);
    }

    if window.len() != period {
        return Err(TechalysisError::BadParam(
            "Window length must match the SMA period".to_string(),
        ));
    }

    for &value in window {
        if value.is_nan() {
            return Err(TechalysisError::UnexpectedNan);
        }
    }

    let mut window = window.clone();

    let old_value = window
        .pop_front()
        .ok_or(TechalysisError::InsufficientData)?;
    window.push_back(new_value);

    let (upper, middle, lower, sum_sq) = bbands_next_unchecked(
        new_value,
        old_value,
        prev_mean,
        prev_sum_sq,
        period as f64,
        std_up,
        std_down,
    );
    Ok(BBandsState {
        upper,
        middle,
        lower,
        sum_sq,
        window,
        period,
        std_up: std_up,
        std_down: std_down,
    })
}

#[inline(always)]
pub fn bbands_next_unchecked(
    new_value: f64,
    old_value: f64,
    prev_mean: f64,
    prev_sum_sq: f64,
    period: f64,
    std_up: f64,
    std_down: f64,
) -> (f64, f64, f64, f64) {
    let sum_sq = prev_sum_sq + (new_value * new_value) - (old_value * old_value);
    let middle = sma_next_unchecked(new_value, old_value, prev_mean, period);
    let upper = to_std_up(middle, sum_sq, period, std_up);
    let lower = to_std_down(middle, sum_sq, period, std_down);
    (upper, middle, lower, sum_sq)
}

#[inline(always)]
fn to_std_up(mean: f64, running_sum_sq: f64, period_as_f64: f64, multiplier: f64) -> f64 {
    mean + multiplier * ((running_sum_sq / period_as_f64) - mean.powi(2)).sqrt()
}

#[inline(always)]
fn to_std_down(mean: f64, running_sum_sq: f64, period_as_f64: f64, multiplier: f64) -> f64 {
    mean - multiplier * ((running_sum_sq / period_as_f64) - mean.powi(2)).sqrt()
}
