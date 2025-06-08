use crate::errors::TechalysisError;
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};
use crate::indicators::sma::sma_next_unchecked;
use crate::types::Float;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct BBandsResult {
    pub upper: Vec<Float>,
    pub middle: Vec<Float>,
    pub lower: Vec<Float>,
    pub state: BBandsState,
}

#[derive(Debug, Clone)]
pub struct BBandsState {
    pub upper: Float,
    pub middle: Float,
    pub lower: Float,
    pub ma: MovingAverageState,
    pub window: VecDeque<Float>,
    pub period: usize,
    pub std: DeviationMulipliers,
    pub ma_type: BBandsMA,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviationMulipliers {
    pub up: Float,
    pub down: Float,
}

#[derive(Debug, Clone, Copy)]
pub struct MovingAverageState {
    pub sma: Float,
    pub ma_sq: Float,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBandsMA {
    SMA,
    EMA(Option<Float>),
}

impl From<BBandsResult> for (Vec<Float>, Vec<Float>, Vec<Float>) {
    fn from(result: BBandsResult) -> Self {
        (result.upper, result.middle, result.lower)
    }
}

impl BBandsState {
    pub fn next(&self, new_value: Float) -> Result<BBandsState, TechalysisError> {
        bbands_next(
            new_value,
            self.middle,
            self.ma,
            &self.window,
            self.period,
            self.std,
            self.ma_type,
        )
    }
}

pub fn bbands_next(
    new_value: Float,
    prev_middle: Float,
    moving_avgs: MovingAverageState,
    window: &VecDeque<Float>,
    period: usize,
    std: DeviationMulipliers,
    ma_type: BBandsMA,
) -> Result<BBandsState, TechalysisError> {
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
    if std.up <= 0.0 || std.down <= 0.0 {
        return Err(TechalysisError::BadParam(
            "Standard deviations must be greater than 0".to_string(),
        ));
    }
    if !moving_avgs.sma.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_sma = {:?}",
            moving_avgs.sma
        )));
    }
    if !prev_middle.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_ma = {prev_middle:?}"
        )));
    }
    if !moving_avgs.ma_sq.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_ma_sq = {:?}",
            moving_avgs.ma_sq
        )));
    }
    if !std.up.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "std_up = {:?}",
            std.up
        )));
    }
    if !std.down.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "std_down = {:?}",
            std.down
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

    let (upper, middle, lower, ma_sq, sma) = match ma_type {
        BBandsMA::SMA => bbands_sma_next_unchecked(
            new_value,
            old_value,
            prev_middle,
            moving_avgs.ma_sq,
            std,
            1.0 / period as Float,
        ),
        BBandsMA::EMA(alpha) => {
            let alpha = if let Some(value) = alpha {
                value
            } else {
                period_to_alpha(period, None)?
            };
            bbands_ema_next_unchecked(
                new_value,
                old_value,
                prev_middle,
                moving_avgs,
                alpha,
                std,
                1.0 / period as Float,
            )
        }
    };

    if !upper.is_finite() {
        return Err(TechalysisError::Overflow(0, upper));
    }
    if !middle.is_finite() {
        return Err(TechalysisError::Overflow(0, middle));
    }
    if !lower.is_finite() {
        return Err(TechalysisError::Overflow(0, lower));
    }

    Ok(BBandsState {
        upper,
        middle,
        lower,
        ma: MovingAverageState { sma, ma_sq },
        window,
        period,
        std,
        ma_type,
    })
}

pub fn bbands(
    data_array: &[Float],
    period: usize,
    std: DeviationMulipliers,
    ma_type: BBandsMA,
) -> Result<BBandsResult, TechalysisError> {
    let mut output_upper = vec![0.0; data_array.len()];
    let mut output_middle = vec![0.0; data_array.len()];
    let mut output_lower = vec![0.0; data_array.len()];

    let bbands_state = bbands_into(
        data_array,
        period,
        std,
        ma_type,
        output_upper.as_mut_slice(),
        output_middle.as_mut_slice(),
        output_lower.as_mut_slice(),
    )?;

    Ok(BBandsResult {
        upper: output_upper,
        middle: output_middle,
        lower: output_lower,
        state: bbands_state,
    })
}

pub fn bbands_into(
    data: &[Float],
    period: usize,
    std: DeviationMulipliers,
    ma_type: BBandsMA,
    output_upper: &mut [Float],
    output_middle: &mut [Float],
    output_lower: &mut [Float],
) -> Result<BBandsState, TechalysisError> {
    let len = data.len();
    let inv_period = 1.0 / (period as Float);
    if period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if std.up <= 0.0 || std.down <= 0.0 {
        return Err(TechalysisError::BadParam(
            "Standard deviations must be greater than 0".to_string(),
        ));
    }

    if output_upper.len() != len || output_middle.len() != len || output_lower.len() != len {
        return Err(TechalysisError::BadParam(
            "Output arrays must have the same length as input data".to_string(),
        ));
    }

    let ma_sq = init_state_unchecked(
        data,
        period,
        inv_period,
        std,
        output_upper,
        output_middle,
        output_lower,
    )?;

    let mut ma = MovingAverageState {
        sma: output_middle[period - 1],
        ma_sq,
    };
    match ma_type {
        BBandsMA::SMA => {
            for idx in period..len {
                if !data[idx].is_finite() {
                    return Err(TechalysisError::DataNonFinite(format!(
                        "data[{idx}] = {:?}",
                        data[idx]
                    )));
                }
                (
                    output_upper[idx],
                    output_middle[idx],
                    output_lower[idx],
                    ma.ma_sq,
                    ma.sma,
                ) = bbands_sma_next_unchecked(
                    data[idx],
                    data[idx - period],
                    output_middle[idx - 1],
                    ma.ma_sq,
                    std,
                    inv_period,
                );
            }
        }
        BBandsMA::EMA(alpha) => {
            let alpha = if let Some(value) = alpha {
                value
            } else {
                period_to_alpha(period, None)?
            };
            for idx in period..len {
                if !data[idx].is_finite() {
                    return Err(TechalysisError::DataNonFinite(format!(
                        "data[{idx}] = {:?}",
                        data[idx]
                    )));
                }
                (
                    output_upper[idx],
                    output_middle[idx],
                    output_lower[idx],
                    ma.ma_sq,
                    ma.sma,
                ) = bbands_ema_next_unchecked(
                    data[idx],
                    data[idx - period],
                    output_middle[idx - 1],
                    ma,
                    alpha,
                    std,
                    inv_period,
                );
                if !output_upper[idx].is_finite() {
                    return Err(TechalysisError::Overflow(idx, output_upper[idx]));
                }
                if !output_middle[idx].is_finite() {
                    return Err(TechalysisError::Overflow(idx, output_middle[idx]));
                }
                if !output_lower[idx].is_finite() {
                    return Err(TechalysisError::Overflow(idx, output_lower[idx]));
                }
            }
        }
    }

    Ok(BBandsState {
        upper: output_upper[len - 1],
        middle: output_middle[len - 1],
        lower: output_lower[len - 1],
        ma,
        window: VecDeque::from(data[len - period..len].to_vec()),
        period,
        std,
        ma_type,
    })
}

#[inline(always)]
pub fn bbands_sma_next_unchecked(
    new_value: Float,
    old_value: Float,
    prev_ma: Float,
    prev_ma_sq: Float,
    std: DeviationMulipliers,
    inv_period: Float,
) -> (Float, Float, Float, Float, Float) {
    let ma_sq = sma_next_unchecked(
        new_value * new_value,
        old_value * old_value,
        prev_ma_sq,
        inv_period,
    );
    let middle = sma_next_unchecked(new_value, old_value, prev_ma, inv_period);
    let (upper, lower) = bands(middle, middle, ma_sq, std.up, std.down);
    (upper, middle, lower, ma_sq, middle)
}

#[inline(always)]
pub fn bbands_ema_next_unchecked(
    new_value: Float,
    old_value: Float,
    prev_middle: Float,
    moving_avgs: MovingAverageState,
    alpha: Float,
    std: DeviationMulipliers,
    inv_period: Float,
) -> (Float, Float, Float, Float, Float) {
    let sma_sq = sma_next_unchecked(
        new_value * new_value,
        old_value * old_value,
        moving_avgs.ma_sq,
        inv_period,
    );
    let sma: Float = sma_next_unchecked(new_value, old_value, moving_avgs.sma, inv_period);
    let middle = ema_next_unchecked(new_value, prev_middle, alpha);
    let (upper, lower) = bands(middle, sma, sma_sq, std.up, std.down);
    (upper, middle, lower, sma_sq, sma)
}

#[inline(always)]
fn bands(
    middle: Float,
    mean: Float,
    mean_sq: Float,
    std_up: Float,
    std_down: Float,
) -> (Float, Float) {
    let std = (mean_sq - mean * mean).abs().sqrt();
    (middle + std_up * std, middle - std_down * std)
}

#[inline(always)]
fn init_state_unchecked(
    data: &[Float],
    period: usize,
    inv_period: Float,
    std: DeviationMulipliers,
    output_upper: &mut [Float],
    output_middle: &mut [Float],
    output_lower: &mut [Float],
) -> Result<Float, TechalysisError> {
    let (mut sum, mut sum_sq) = (0.0, 0.0);
    for idx in 0..period {
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{idx}] = {:?}",
                value
            )));
        } else {
            sum += value;
            sum_sq += value * value;
        }
        output_upper[idx] = Float::NAN;
        output_middle[idx] = Float::NAN;
        output_lower[idx] = Float::NAN;
    }
    output_middle[period - 1] = sum * inv_period;
    let ma_sq = sum_sq * inv_period;
    (output_upper[period - 1], output_lower[period - 1]) = bands(
        output_middle[period - 1],
        output_middle[period - 1],
        ma_sq,
        std.up,
        std.down,
    );
    if !output_middle[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(
            period - 1,
            output_middle[period - 1],
        ));
    }
    if !output_upper[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(
            period - 1,
            output_upper[period - 1],
        ));
    }
    if !output_lower[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(
            period - 1,
            output_lower[period - 1],
        ));
    }
    Ok(ma_sq)
}
