use std::collections::VecDeque;

use crate::errors::TechalysisError;
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};
use crate::indicators::sma::sma_next_unchecked;

#[derive(Debug)]
pub struct BBandsResult {
    pub upper_band: Vec<f64>,
    pub middle_band: Vec<f64>,
    pub lower_band: Vec<f64>,
    pub state: BBandsState,
}

#[derive(Debug, Clone)]
pub struct BBandsState {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub sma: f64,
    pub ma_sq: f64,
    pub window: VecDeque<f64>,
    pub period: usize,
    pub std_up: f64,
    pub std_down: f64,
    pub ma_type: BBandsMA,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBandsMA {
    SMA,
    EMA(Option<f64>),
}

impl From<BBandsResult> for (Vec<f64>, Vec<f64>, Vec<f64>) {
    fn from(result: BBandsResult) -> Self {
        (result.upper_band, result.middle_band, result.lower_band)
    }
}

impl BBandsState {
    pub fn next(&self, new_value: f64) -> Result<BBandsState, TechalysisError> {
        bbands_next(
            new_value,
            self.sma,
            self.middle,
            self.ma_sq,
            &self.window,
            self.period,
            self.std_up,
            self.std_down,
            self.ma_type,
        )
    }
}

pub fn bbands_next(
    new_value: f64,
    prev_sma: f64,
    prev_ma: f64,
    prev_ma_sq: f64,
    window: &VecDeque<f64>,
    period: usize,
    std_up: f64,
    std_down: f64,
    ma_type: BBandsMA,
) -> Result<BBandsState, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if new_value.is_nan() || prev_ma.is_nan() || prev_ma_sq.is_nan() {
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

    let (upper, middle, lower, ma_sq, sma) = match ma_type {
        BBandsMA::SMA => bbands_sma_next_unchecked(
            new_value,
            old_value,
            prev_ma,
            prev_ma_sq,
            std_up,
            std_down,
            1.0 / period as f64,
        ),
        BBandsMA::EMA(alpha) => {
            let alpha = if alpha.is_none() {
                period_to_alpha(period, None)?
            } else {
                alpha.unwrap()
            };
            bbands_ema_next_unchecked(
                new_value,
                old_value,
                prev_sma,
                prev_ma,
                prev_ma_sq,
                alpha,
                std_up,
                std_down,
                1.0 / period as f64,
            )
        }
    };

    Ok(BBandsState {
        upper,
        middle,
        lower,
        sma,
        ma_sq,
        window,
        period,
        std_up: std_up,
        std_down: std_down,
        ma_type,
    })
}

pub fn bbands(
    data_array: &[f64],
    period: usize,
    std_up: f64,
    std_down: f64,
    ma_type: BBandsMA,
) -> Result<BBandsResult, TechalysisError> {
    let mut output_upper = vec![0.0; data_array.len()];
    let mut output_middle = vec![0.0; data_array.len()];
    let mut output_lower = vec![0.0; data_array.len()];

    let bbands_state = bbands_into(
        data_array,
        period,
        std_up,
        std_down,
        ma_type,
        output_upper.as_mut_slice(),
        output_middle.as_mut_slice(),
        output_lower.as_mut_slice(),
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
    ma_type: BBandsMA,
    output_upper: &mut [f64],
    output_middle: &mut [f64],
    output_lower: &mut [f64],
) -> Result<BBandsState, TechalysisError> {
    let len = data_array.len();
    let inv_period = 1.0 / (period as f64);
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

    let mut ma_sq = init_state_unchecked(
        data_array,
        period,
        inv_period,
        std_up,
        std_down,
        output_upper,
        output_middle,
        output_lower,
    )?;

    let mut sma = output_middle[period - 1];
    match ma_type {
        BBandsMA::SMA => {
            for idx in period..len {
                if data_array[idx].is_nan() {
                    return Err(TechalysisError::UnexpectedNan);
                }
                (
                    output_upper[idx],
                    output_middle[idx],
                    output_lower[idx],
                    ma_sq,
                    sma,
                ) = bbands_sma_next_unchecked(
                    data_array[idx],
                    data_array[idx - period],
                    output_middle[idx - 1],
                    ma_sq,
                    std_up,
                    std_down,
                    inv_period,
                );
            }
        }
        BBandsMA::EMA(alpha) => {
            let alpha = if alpha.is_none() {
                period_to_alpha(period, None)?
            } else {
                alpha.unwrap()
            };
            for idx in period..len {
                if data_array[idx].is_nan() {
                    return Err(TechalysisError::UnexpectedNan);
                }
                (
                    output_upper[idx],
                    output_middle[idx],
                    output_lower[idx],
                    ma_sq,
                    sma,
                ) = bbands_ema_next_unchecked(
                    data_array[idx],
                    data_array[idx - period],
                    sma,
                    output_middle[idx - 1],
                    ma_sq,
                    alpha,
                    std_up,
                    std_down,
                    inv_period,
                );
            }
        }
    }

    Ok(BBandsState {
        upper: output_upper[len - 1],
        middle: output_middle[len - 1],
        lower: output_lower[len - 1],
        sma,
        ma_sq,
        window: VecDeque::from(data_array[len - period..len].to_vec()),
        period,
        std_up,
        std_down,
        ma_type,
    })
}

#[inline(always)]
pub fn bbands_sma_next_unchecked(
    new_value: f64,
    old_value: f64,
    prev_ma: f64,
    prev_ma_sq: f64,
    std_up: f64,
    std_down: f64,
    inv_period: f64,
) -> (f64, f64, f64, f64, f64) {
    let ma_sq = sma_next_unchecked(
        new_value * new_value,
        old_value * old_value,
        prev_ma_sq,
        inv_period,
    );
    let middle = sma_next_unchecked(new_value, old_value, prev_ma, inv_period);
    let (upper, lower) = bands(middle, middle, ma_sq, std_up, std_down);
    (upper, middle, lower, ma_sq, middle)
}

#[inline(always)]
pub fn bbands_ema_next_unchecked(
    new_value: f64,
    old_value: f64,
    prev_sma: f64,
    prev_ema: f64,
    prev_sma_sq: f64,
    alpha: f64,
    std_up: f64,
    std_down: f64,
    inv_period: f64,
) -> (f64, f64, f64, f64, f64) {
    let sma_sq = sma_next_unchecked(
        new_value * new_value,
        old_value * old_value,
        prev_sma_sq,
        inv_period,
    );
    let sma: f64 = sma_next_unchecked(new_value, old_value, prev_sma, inv_period);
    let middle = ema_next_unchecked(new_value, prev_ema, alpha);
    let (upper, lower) = bands(middle, sma, sma_sq, std_up, std_down);
    (upper, middle, lower, sma_sq, sma)
}

#[inline(always)]
fn bands(middle: f64, mean: f64, mean_sq: f64, std_up: f64, std_down: f64) -> (f64, f64) {
    let std = (mean_sq - mean * mean).abs().sqrt();
    (middle + std_up * std, middle - std_down * std)
}

#[inline(always)]
fn init_state_unchecked(
    data_array: &[f64],
    period: usize,
    inv_period: f64,
    std_up: f64,
    std_down: f64,
    output_upper: &mut [f64],
    output_middle: &mut [f64],
    output_lower: &mut [f64],
) -> Result<f64, TechalysisError> {
    let (mut sum, mut sum_sq) = (0.0, 0.0);
    for idx in 0..period {
        let value = &data_array[idx];
        if value.is_nan() {
            return Err(TechalysisError::UnexpectedNan);
        } else {
            sum += value;
            sum_sq += value * value;
        }
        output_upper[idx] = f64::NAN;
        output_middle[idx] = f64::NAN;
        output_lower[idx] = f64::NAN;
    }
    output_middle[period - 1] = sum * inv_period;
    let ma_sq = sum_sq * inv_period;
    (output_upper[period - 1], output_lower[period - 1]) = bands(
        output_middle[period - 1],
        output_middle[period - 1],
        ma_sq,
        std_up,
        std_down,
    );
    Ok(ma_sq)
}
