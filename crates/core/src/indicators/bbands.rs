/*
    BSD 3-Clause License

    Copyright (c) 2025, Guillaume GOBIN (Guitheg)

    Redistribution and use in source and binary forms, with or without modification,
    are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation and/or
    other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors
    may be used to endorse or promote products derived from this software without
    specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
    THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

/*
    List of contributors:
    - Guitheg: Initial implementation
*/

/*
    Inspired by TA-LIB BBANDS implementation
*/

//! Bollinger Bands (BBANDS) implementation

use crate::errors::TechalibError;
use crate::indicators::ema::{ema_next_unchecked, period_to_alpha};
use crate::indicators::sma::sma_next_unchecked;
use crate::traits::State;
use crate::types::Float;
use std::collections::VecDeque;

/// Bollinger Bands result
/// ---
/// This struct holds the result of the Bollinger Bands calculation.
/// It contains the upper, middle, and lower bands as well as the state of the calculation.
///
/// Attributes
/// ---
/// - `upper`: The upper Bollinger Band values.
/// - `middle`: The middle Bollinger Band values (usually a moving average).
/// - `lower`: The lower Bollinger Band values.
/// - `state`: A [`BBandsState`], which can be used to calculate the next values
///   incrementally.
#[derive(Debug)]
pub struct BBandsResult {
    /// The upper Bollinger Band values.
    pub upper: Vec<Float>,
    /// The middle Bollinger Band values (usually a moving average).
    pub middle: Vec<Float>,
    /// The lower Bollinger Band values.
    pub lower: Vec<Float>,
    /// A [`BBandsState`], which can be used to calculate the next values
    /// incrementally.
    pub state: BBandsState,
}

/// Bollinger Bands calculation state
/// ---
/// This struct holds the state of the Bollinger Bands calculation.
/// It is used to calculate the next values in the Bollinger Bands series in
/// an incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `upper`: The last upper Bollinger Band value.
/// - `middle`: The last middle Bollinger Band value (usually a moving average).
/// - `lower`: The last lower Bollinger Band value.
///
/// **State values**
/// - `moving_averages`: The state of the moving averages used in the calculation.
/// - `last_window`: A deque containing the last `period` values used for the calculation.
///
/// **Parameters**
/// - `period`: The number of periods used to calculate the moving average and
///   standard deviation.
/// - `std_dev_mult`: The multipliers for the standard deviation used to calculate
///   the upper and lower bands.
/// - `ma_type`: The type of moving average used (SMA or EMA).
#[derive(Debug, Clone)]
pub struct BBandsState {
    // Outputs values
    /// The last upper Bollinger Band value.
    pub upper: Float,
    /// The last middle Bollinger Band value (usually a moving average).
    pub middle: Float,
    /// The last lower Bollinger Band value.
    pub lower: Float,

    // State values
    /// The [`MovingAverageState`] state of the moving averages used in the calculation.
    pub moving_averages: MovingAverageState,
    /// A deque containing the last `period` values used for the calculation.
    pub last_window: VecDeque<Float>,

    // Parameters
    /// The number of periods used to calculate the moving average and standard deviation.
    pub period: usize,
    /// The multipliers for the standard deviation used to calculate the upper and lower bands.
    pub std_dev_mult: DeviationMulipliers,
    /// The [`BBandsMA`] enum variant representing the type of moving average used.
    pub ma_type: BBandsMA,
}

/// Deviation multipliers for Bollinger Bands.
/// ---
///
/// This struct holds the multipliers for the standard deviation used to calculate the upper and lower Bollinger Bands.
///
/// Attributes
/// ---
/// - `up`: The multiplier for the upper Bollinger Band.
/// - `down`: The multiplier for the lower Bollinger Band.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviationMulipliers {
    /// The multiplier for the upper Bollinger Band.
    pub up: Float,
    /// The multiplier for the lower Bollinger Band.
    pub down: Float,
}

/// Moving average state for Bollinger Bands.
/// ---
///
/// This struct holds the state of the moving averages used in the Bollinger Bands calculation.
///
/// Attributes
/// ---
/// - `sma`: The simple moving average value.
/// - `ma_square`: The square of the moving average value, used for variance calculation.
///   The moving average depends on the `ma_type` used in the Bollinger Bands calculation.
#[derive(Debug, Clone, Copy)]
pub struct MovingAverageState {
    /// The simple moving average value.
    pub sma: Float,
    /// The square of the moving average value, used for variance calculation.
    /// The moving average depends on the `ma_type` used in the Bollinger Bands calculation.
    /// This value is used to calculate the standard deviation and is essential for
    /// determining the upper and lower Bollinger Bands.
    pub ma_square: Float,
}

/// Type of moving average used in Bollinger Bands.
/// ---
///
/// This enum defines the type of moving average used in the Bollinger Bands calculation.
///
/// Variants
/// ---
/// - `SMA`: Simple Moving Average.
/// - `EMA`: Exponential Moving Average, with an optional alpha value for the calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBandsMA {
    /// Simple Moving Average.
    SMA,
    /// Exponential Moving Average, with an optional alpha value for the calculation.
    EMA(Option<Float>),
}

impl State<Float> for BBandsState {
    /// Update the [`BBandsState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input value to update the Bollinger Bands state. Generally, it is the closing price.
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        if self.period <= 1 {
            return Err(TechalibError::BadParam(
                "SMA period must be greater than 1".to_string(),
            ));
        }
        if !sample.is_finite() {
            return Err(TechalibError::DataNonFinite(format!("sample = {sample:?}")));
        }
        if self.std_dev_mult.up <= 0.0 || self.std_dev_mult.down <= 0.0 {
            return Err(TechalibError::BadParam(
                "Standard deviations must be greater than 0".to_string(),
            ));
        }
        if !self.moving_averages.sma.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.moving_averages.sma = {:?}",
                self.moving_averages.sma
            )));
        }
        if !self.middle.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.middle = {:?}",
                self.middle
            )));
        }
        if !self.moving_averages.ma_square.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.moving_averages.ma_square = {:?}",
                self.moving_averages.ma_square
            )));
        }
        if !self.std_dev_mult.up.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.std_dev_mult.up = {:?}",
                self.std_dev_mult.up
            )));
        }
        if !self.std_dev_mult.down.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "self.std_dev_mult.down = {:?}",
                self.std_dev_mult.down
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalibError::BadParam(
                "Window length must match the SMA period".to_string(),
            ));
        }

        for (idx, &value) in self.last_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalibError::DataNonFinite(format!(
                    "window[{idx}] = {value:?}"
                )));
            }
        }

        let mut window = self.last_window.clone();

        let old_value = window.pop_front().ok_or(TechalibError::InsufficientData)?;
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
            return Err(TechalibError::Overflow(0, upper));
        }
        if !middle.is_finite() {
            return Err(TechalibError::Overflow(0, middle));
        }
        if !lower.is_finite() {
            return Err(TechalibError::Overflow(0, lower));
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

/// Calculate Bollinger Bands for a given data array and return the result.
///
/// Input Arguments
/// ---
/// - `data`: A slice of `Float` values representing the data to calculate the Bollinger Bands on.
/// - `period`: The time period over which to calculate the Bollinger Bands.
/// - `std_dev_mul`: A struct containing the multipliers for the standard deviation used to calculate the upper and lower bands.
/// - `ma_type`: The type of moving average to use (SMA or EMA).
///
/// Returns
/// ---
/// A `Result` containing a [`BBandsResult`] with the upper, middle, and lower bands,
/// or an error if the calculation fails.
pub fn bbands(
    data: &[Float],
    period: usize,
    std_dev_mul: DeviationMulipliers,
    ma_type: BBandsMA,
) -> Result<BBandsResult, TechalibError> {
    let mut output_upper = vec![0.0; data.len()];
    let mut output_middle = vec![0.0; data.len()];
    let mut output_lower = vec![0.0; data.len()];

    let bbands_state = bbands_into(
        data,
        period,
        std_dev_mul,
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

/// Calculate Bollinger Bands and store the results in provided output arrays and return the state.
///
/// Input Arguments
/// ---
/// - `data`: A slice of `Float` values representing the data to calculate the Bollinger Bands on.
/// - `period`: The time period over which to calculate the Bollinger Bands.
/// - `std_dev_mul`: A struct containing the multipliers for the standard deviation used to calculate the upper and lower bands.
/// - `ma_type`: The type of moving average to use (SMA or EMA).
///
/// Output Arguments
/// ---
/// - `output_upper`: A mutable slice to store the upper Bollinger Band values.
/// - `output_middle`: A mutable slice to store the middle Bollinger Band values.
/// - `output_lower`: A mutable slice to store the lower Bollinger Band values.
///
/// Returns
/// ---
/// A `Result` containing a [`BBandsState`] with the last calculated values and state, or an error if the calculation fails.
pub fn bbands_into(
    data: &[Float],
    period: usize,
    std_dev_mul: DeviationMulipliers,
    ma_type: BBandsMA,
    output_upper: &mut [Float],
    output_middle: &mut [Float],
    output_lower: &mut [Float],
) -> Result<BBandsState, TechalibError> {
    let len = data.len();
    let inv_period = 1.0 / (period as Float);
    if period > len {
        return Err(TechalibError::InsufficientData);
    }

    if period <= 1 {
        return Err(TechalibError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    if std_dev_mul.up <= 0.0 || std_dev_mul.down <= 0.0 {
        return Err(TechalibError::BadParam(
            "Standard deviations must be greater than 0".to_string(),
        ));
    }

    if output_upper.len() != len || output_middle.len() != len || output_lower.len() != len {
        return Err(TechalibError::BadParam(
            "Output arrays must have the same length as input data".to_string(),
        ));
    }

    let ma_sq = init_state_unchecked(
        data,
        period,
        inv_period,
        std_dev_mul,
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
                    return Err(TechalibError::DataNonFinite(format!(
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
                    std_dev_mul,
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
                    return Err(TechalibError::DataNonFinite(format!(
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
                    std_dev_mul,
                    inv_period,
                );
                if !output_upper[idx].is_finite() {
                    return Err(TechalibError::Overflow(idx, output_upper[idx]));
                }
                if !output_middle[idx].is_finite() {
                    return Err(TechalibError::Overflow(idx, output_middle[idx]));
                }
                if !output_lower[idx].is_finite() {
                    return Err(TechalibError::Overflow(idx, output_lower[idx]));
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
        std_dev_mult: std_dev_mul,
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
) -> Result<Float, TechalibError> {
    let (mut sum, mut sum_sq) = (0.0, 0.0);
    for idx in 0..period {
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
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
        return Err(TechalibError::Overflow(
            period - 1,
            output_middle[period - 1],
        ));
    }
    if !output_upper[period - 1].is_finite() {
        return Err(TechalibError::Overflow(
            period - 1,
            output_upper[period - 1],
        ));
    }
    if !output_lower[period - 1].is_finite() {
        return Err(TechalibError::Overflow(
            period - 1,
            output_lower[period - 1],
        ));
    }
    Ok(ma_sq)
}
