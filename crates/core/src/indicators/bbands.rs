// BSD 3-Clause License

// Copyright (c) 2025, Guillaume GOBIN (Guitheg)

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.

// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.

// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::errors::TechalysisError;
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};
use crate::indicators::sma::sma_next_unchecked;
use crate::traits::State;
use crate::types::Float;
use std::collections::VecDeque;

/// Bollinger Bands result.
/// ---
/// This struct holds the result of the Bollinger Bands calculation.
/// It contains the upper, middle, and lower bands as well as the state of the calculation.
/// 
/// Attributes:
/// ---
/// - `upper`: The upper Bollinger Band values.
/// - `middle`: The middle Bollinger Band values (usually a moving average).
/// - `lower`: The lower Bollinger Band values.
/// - `state`: The state of the Bollinger Bands calculation, which can be used to calculate the next values incrementally.
#[derive(Debug)]
pub struct BBandsResult {
    pub upper: Vec<Float>,
    pub middle: Vec<Float>,
    pub lower: Vec<Float>,
    pub state: BBandsState,
}


/// Bollinger Bands calculation state.
/// ---
/// This struct holds the state of the Bollinger Bands calculation.
/// It is used to calculate the next values in the Bollinger Bands series in a incremental way.
/// 
/// Attributes:
/// ---
/// Last outputs values:
/// - `upper`: The last upper Bollinger Band value.
/// - `middle`: The last middle Bollinger Band value (usually a moving average).
/// - `lower`: The last lower Bollinger Band value.
/// 
/// State values:
/// - `moving_averages`: The state of the moving averages used in the calculation.
/// - `last_window`: A deque containing the last `period` values used for the calculation.
/// 
/// Parameters:
/// - `period`: The number of periods used to calculate the moving average and standard deviation.
/// - `std_dev_mult`: The multipliers for the standard deviation used to calculate the upper and lower bands.
/// - `ma_type`: The type of moving average used (SMA or EMA).
#[derive(Debug, Clone)]
pub struct BBandsState {
    // Outputs values
    pub upper: Float,
    pub middle: Float,
    pub lower: Float,

    // State values
    pub moving_averages: MovingAverageState,
    pub last_window: VecDeque<Float>,

    // Parameters
    pub period: usize,
    pub std_dev_mult: DeviationMulipliers,
    pub ma_type: BBandsMA,
}

/// Deviation multipliers for Bollinger Bands.
/// ---
/// 
/// This struct holds the multipliers for the standard deviation used to calculate the upper and lower Bollinger Bands.
/// 
/// Attributes:
/// ---
/// - `up`: The multiplier for the upper Bollinger Band.
/// - `down`: The multiplier for the lower Bollinger Band.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviationMulipliers {
    pub up: Float,
    pub down: Float,
}

/// Moving average state for Bollinger Bands.
/// ---
/// 
/// This struct holds the state of the moving averages used in the Bollinger Bands calculation.
/// 
/// Attributes:
/// ---
/// - `sma`: The simple moving average value.
/// - `ma_square`: The square of the moving average value, used for variance calculation.
/// The moving average depends on the `ma_type` used in the Bollinger Bands calculation.
#[derive(Debug, Clone, Copy)]
pub struct MovingAverageState {
    pub sma: Float,
    pub ma_square: Float,
}

/// Type of moving average used in Bollinger Bands.
/// ---
/// 
/// This enum defines the type of moving average used in the Bollinger Bands calculation.
/// 
/// Variants:
/// - `SMA`: Simple Moving Average.
/// - `EMA`: Exponential Moving Average, with an optional alpha value for the calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBandsMA {
    SMA,
    EMA(Option<Float>),
}

impl State<Float> for BBandsState {
    /// Calculates the next [`BBandsState`] based on a new input value.
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
        if self.std_dev_mult.up <= 0.0 || self.std_dev_mult.down <= 0.0 {
            return Err(TechalysisError::BadParam(
                "Standard deviations must be greater than 0".to_string(),
            ));
        }
        if !self.moving_averages.sma.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.moving_averages.sma = {:?}",
                self.moving_averages.sma
            )));
        }
        if !self.middle.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.middle = {:?}", self.middle
            )));
        }
        if !self.moving_averages.ma_square.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.moving_averages.ma_square = {:?}",
                self.moving_averages.ma_square
            )));
        }
        if !self.std_dev_mult.up.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.std_dev_mult.up = {:?}",
                self.std_dev_mult.up
            )));
        }
        if !self.std_dev_mult.down.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "self.std_dev_mult.down = {:?}",
                self.std_dev_mult.down
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalysisError::BadParam(
                "Window length must match the SMA period".to_string(),
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

        let (upper, middle, lower, ma_sq, sma) = match self.ma_type {
            BBandsMA::SMA => bbands_sma_next_unchecked(
                sample,
                old_value,
                self.middle,
                self.moving_averages.ma_square,
                self.std_dev_mult,
                1.0 / self.period as Float,
            ),
            BBandsMA::EMA(alpha) => {
                let alpha = if let Some(value) = alpha {
                    value
                } else {
                    period_to_alpha(self.period, None)?
                };
                bbands_ema_next_unchecked(
                    sample,
                    old_value,
                    self.middle,
                    self.moving_averages,
                    alpha,
                    self.std_dev_mult,
                    1.0 / self.period as Float,
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

        self.upper = upper;
        self.middle = middle;
        self.lower = lower;
        self.moving_averages.sma = sma;
        self.moving_averages.ma_square = ma_sq;
        self.last_window = window;
        Ok(())
    }
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
        ma_square: ma_sq,
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
                    ma.ma_square,
                    ma.sma,
                ) = bbands_sma_next_unchecked(
                    data[idx],
                    data[idx - period],
                    output_middle[idx - 1],
                    ma.ma_square,
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
                    ma.ma_square,
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
        moving_averages: ma,
        last_window: VecDeque::from(data[len - period..len].to_vec()),
        period,
        std_dev_mult: std,
        ma_type,
    })
}

#[inline(always)]
fn bbands_sma_next_unchecked(
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
fn bbands_ema_next_unchecked(
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
        moving_avgs.ma_square,
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
