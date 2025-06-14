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
    Inspired by TA-LIB TRIMA implementation
*/

//! Triangular Moving Average (TRIMA) implementation

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;
use std::collections::VecDeque;

/// TRIMA calculation result
/// ---
/// This struct holds the result and the state ([`TrimaState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `values`: A vector of [`Float`] representing the calculated TRIMA values.
/// - `state`: A [`TrimaState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct TrimaResult {
    /// The calculated TRIMA values.
    pub values: Vec<Float>,
    /// A [`TrimaState`], which can be used to calculate
    /// the next values incrementally.
    pub state: TrimaState,
}

/// TRIMA calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Last outputs values**
/// - `trima`: The last calculated TRIMA value.
///
/// **State values**
/// - `weighted_sum`: The weighted sum of the values in the current window.
/// - `trailing_sum`: The sum of the first half of the values in
///   the current window. It is used to optimize the calculation of the TRIMA.
/// - `heading_sum`: The sum of the second half of the values in the current window.
///   It is used to optimize the calculation of the TRIMA.
/// - `last_window`: A deque containing the last `period` values used for
///   the TRIMA calculation.
///
/// **Parameters**
/// - `inv_weight_sum`: The inverse of the sum of weights used in the TRIMA calculation.
/// - `period`: The period used for the TRIMA calculation, which determines
///   how many values are averaged to compute the TRIMA.
#[derive(Debug, Clone)]
pub struct TrimaState {
    // Outputs
    /// The last calculated TRIMA value
    pub trima: Float,

    // State values
    /// The weighted sum of the values in the current window
    pub weighted_sum: Float,
    /// The sum of the first half of the values in the current window.
    pub trailing_sum: Float,
    /// The sum of the second half of the values in the current window.
    pub heading_sum: Float,
    /// A deque containing the last `period` values used for
    pub last_window: VecDeque<Float>,

    // Parameters
    /// The inverse of the sum of weights used in the TRIMA calculation
    /// It is calculated as `1.0 / ((period // 2) * (period // 2 + 1))` for even periods
    /// and `1.0 / ((period // 2 + 1) * (period // 2 + 1))` for odd periods.
    /// This value is used to optimize the calculation of the TRIMA.
    pub inv_weight_sum: Float,
    /// The period used for the TRIMA calculation, which determines
    pub period: usize,
}

impl State<Float> for TrimaState {
    /// Update the [`TrimaState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the TRIMA state
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        TechalibError::check_period(self.period)?;
        TechalibError::check_finite(sample, "sample")?;
        if !self.trima.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "prev_trima = {:?}",
                self.trima
            )));
        }
        if self.last_window.len() != self.period {
            return Err(TechalibError::BadParam(
                "Window length must match the TRIMA period".to_string(),
            ));
        }

        for (idx, &value) in self.last_window.iter().enumerate() {
            if !value.is_finite() {
                return Err(TechalibError::DataNonFinite(format!(
                    "window[{idx}] = {value:?}"
                )));
            }
        }
        let is_odd = self.period % 2 != 0;

        let mut window = self.last_window.clone();

        let old_value = window.pop_front().ok_or(TechalibError::InsufficientData)?;
        window.push_back(sample);
        let vec = Vec::from(window.clone());
        let middle_idx = get_middle_idx(self.period);
        let middle_value = vec[middle_idx];

        let (trima, new_weighted_sum, new_trailing_sum, new_heading_sum) = if is_odd {
            trima_next_odd_unchecked(
                sample,
                middle_value,
                old_value,
                self.weighted_sum,
                self.trailing_sum,
                self.heading_sum,
                self.inv_weight_sum,
            )
        } else {
            trima_next_even_unchecked(
                sample,
                middle_value,
                old_value,
                self.weighted_sum,
                self.trailing_sum,
                self.heading_sum,
                self.inv_weight_sum,
            )
        };

        TechalibError::check_overflow(trima)?;

        self.trima = trima;
        self.weighted_sum = new_weighted_sum;
        self.trailing_sum = new_trailing_sum;
        self.heading_sum = new_heading_sum;
        self.last_window = window;
        Ok(())
    }
}

/// Lookback period for TRIMA calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n` first values that will be return will be `NaN`
/// and the next values will be the values.
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    TechalibError::check_period(period)?;
    Ok(period - 1)
}

/// Calculation of the TRIMA function
/// ---
/// It returns a [`TrimaResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the TRIMA calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`TrimaResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn trima(data: &[Float], period: usize) -> Result<TrimaResult, TechalibError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let trima_state = trima_into(data, period, &mut output)?;
    Ok(TrimaResult {
        values: output,
        state: trima_state,
    })
}

/// Calculation of the TRIMA function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`TrimaState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the TRIMA calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the calculated TRIMA values
///
/// Returns
/// ---
/// A `Result` containing a [`TrimaState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn trima_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<TrimaState, TechalibError> {
    TechalibError::check_same_length(("data", data), ("output", output))?;
    let len = data.len();
    let is_odd = period % 2 != 0;
    let lookback = lookback_from_period(period)?;
    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let (trima, mut sum, mut trailing_sum, mut heading_sum, inv_weight_sum, mut middle_idx) =
        init_trima_unchecked(data, period, output)?;

    output[lookback] = trima;
    TechalibError::check_overflow_at(lookback, output)?;
    middle_idx += 1;

    if is_odd {
        for idx in period..len {
            TechalibError::check_finite_at(idx, data)?;
            (output[idx], sum, trailing_sum, heading_sum) = trima_next_odd_unchecked(
                data[idx],
                data[middle_idx],
                data[idx - period],
                sum,
                trailing_sum,
                heading_sum,
                inv_weight_sum,
            );
            TechalibError::check_overflow_at(idx, output)?;
            middle_idx += 1;
        }
    } else {
        for idx in period..len {
            TechalibError::check_finite_at(idx, data)?;
            (output[idx], sum, trailing_sum, heading_sum) = trima_next_even_unchecked(
                data[idx],
                data[middle_idx],
                data[idx - period],
                sum,
                trailing_sum,
                heading_sum,
                inv_weight_sum,
            );
            TechalibError::check_overflow_at(idx, output)?;
            middle_idx += 1;
        }
    }

    Ok(TrimaState {
        trima: output[len - 1],
        weighted_sum: sum,
        trailing_sum,
        heading_sum,
        last_window: VecDeque::from(data[len - period..len].to_vec()),
        inv_weight_sum,
        period,
    })
}

#[inline(always)]
fn trima_next_even_unchecked(
    new_value: Float,
    middle_value: Float,
    old_value: Float,
    sum: Float,
    trailing_sum: Float,
    heading_sum: Float,
    inv_weight_sum: Float,
) -> (Float, Float, Float, Float) {
    let new_trailing_sum = trailing_sum - old_value + middle_value;
    let new_heading_sum = heading_sum - middle_value + new_value;
    let new_sum = sum - trailing_sum + new_heading_sum;
    (
        new_sum * inv_weight_sum,
        new_sum,
        new_trailing_sum,
        new_heading_sum,
    )
}

#[inline(always)]
fn trima_next_odd_unchecked(
    new_value: Float,
    middle_value: Float,
    old_value: Float,
    sum: Float,
    trailing_sum: Float,
    heading_sum: Float,
    inv_weight_sum: Float,
) -> (Float, Float, Float, Float) {
    let new_trailing_sum = trailing_sum - old_value + middle_value;
    let new_heading_sum = heading_sum - middle_value + new_value;
    let new_sum = sum - trailing_sum + new_heading_sum + middle_value;
    (
        new_sum * inv_weight_sum,
        new_sum,
        new_trailing_sum,
        new_heading_sum,
    )
}

#[inline(always)]
fn init_trima_unchecked(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<(Float, Float, Float, Float, Float, usize), TechalibError> {
    let middle_idx = get_middle_idx(period);
    let mut trailing_sum: Float = 0.0;
    let mut heading_sum: Float = 0.0;
    let mut sum: Float = 0.0;
    let inv_weight_sum = trima_inv_weight_sum(period)?;
    for idx in 0..=middle_idx {
        let weight = (idx + 1) as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        }
        trailing_sum += value;
        sum += value * weight;
        output[idx] = Float::NAN;
    }
    for (local_idx, idx) in (middle_idx + 1..period).rev().enumerate() {
        let weight = (local_idx + 1) as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalibError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        }
        heading_sum += value;
        sum += value * weight;
        output[idx] = Float::NAN;
    }

    Ok((
        sum * inv_weight_sum,
        sum,
        trailing_sum,
        heading_sum,
        inv_weight_sum,
        middle_idx,
    ))
}

fn trima_inv_weight_sum(period: usize) -> Result<Float, TechalibError> {
    if period <= 1 {
        return Err(TechalibError::BadParam(
            "TRIMA period must be greater than 1".to_string(),
        ));
    }
    let p = (period / 2) as Float;
    if period % 2 == 0 {
        Ok(1.0 / (p * (p + 1.0)))
    } else {
        Ok(1.0 / ((p + 1.0) * (p + 1.0)))
    }
}

fn get_middle_idx(period: usize) -> usize {
    if period % 2 == 0 {
        period / 2 - 1
    } else {
        period / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_trima_unchecked_odd_ok() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let period = 5;
        let mut output = vec![0.0; data.len()];
        let expected_inv_weight_sum = 1.0 / 9.0; // (5//2) * (5//2 + 1) = 9, so inv_weight_sum = 1/9
        let expected_sum = 27.0; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 + 5) = 27
        let expected_trailing_sum = 6.0; // (1 + 2 + 3) = 6, but we only take the first half
        let expected_heading_sum = 9.0; // (4 + 5) = 12, but we only take the second half
        let expected_middle_idx = 2; // middle index for period 5 is 2
        let expected_trima = 3.0; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 + 5) * (1/9) = 3.0

        let (trima, sum, trailing_sum, heading_sum, inv_weight_sum, middle_idx) =
            init_trima_unchecked(&data, period, &mut output).unwrap();

        assert!(
            output.iter().take(period - 1).all(|&v| v.is_nan()),
            "Expected first {} values to be NaN",
            period - 1
        );
        assert!(
            inv_weight_sum == expected_inv_weight_sum,
            "Expected inv_weight_sum to be {}, got {}",
            expected_inv_weight_sum,
            inv_weight_sum
        );
        assert!(
            middle_idx == expected_middle_idx,
            "Expected middle_idx to be {expected_middle_idx:?}, got {}",
            middle_idx
        );
        assert!(
            trailing_sum == expected_trailing_sum,
            "Expected {expected_trailing_sum:?}, got {}",
            trailing_sum
        );
        assert!(
            heading_sum == expected_heading_sum,
            "Expected {expected_heading_sum:?}, got {}",
            heading_sum
        );
        assert!(
            sum == expected_sum,
            "Expected {expected_sum:?}, got {}",
            sum
        );
        assert!(
            trima == expected_trima,
            "Expected trima {expected_trima:?}, got {}",
            trima
        );
    }

    #[test]
    fn init_trima_unchecked_even_ok() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let period = 6;
        let mut output = vec![0.0; data.len()];
        let expected_inv_weight_sum = 1.0 / 12.0; // (6//2) * (6//2 + 1) = 12, so inv_weight_sum = 1/12
        let expected_sum = 42.0; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 +4 + 5 + 5 + 6) = 42
        let expected_trailing_sum = 6.0; // (1 + 2 + 3) = 6, but we only take the first half
        let expected_heading_sum = 15.0; // (4 + 5 + 6) = 15, but we only take the second half
        let expected_middle_idx = 2; // middle index for period 6 is 3
        let expected_trima = 3.5; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 + 4 + 5 + 5 + 6) * (1/12) = 3.5

        let (trima, sum, trailing_sum, heading_sum, inv_weight_sum, middle_idx) =
            init_trima_unchecked(&data, period, &mut output).unwrap();

        assert!(
            output.iter().take(period - 1).all(|&v| v.is_nan()),
            "Expected first {} values to be NaN",
            period - 1
        );
        assert!(
            inv_weight_sum == expected_inv_weight_sum,
            "Expected inv_weight_sum to be {}, got {}",
            expected_inv_weight_sum,
            inv_weight_sum
        );
        assert!(
            middle_idx == expected_middle_idx,
            "Expected middle_idx to be {expected_middle_idx:?}, got {}",
            middle_idx
        );
        assert!(
            trailing_sum == expected_trailing_sum,
            "Expected {expected_trailing_sum:?}, got {}",
            trailing_sum
        );
        assert!(
            heading_sum == expected_heading_sum,
            "Expected {expected_heading_sum:?}, got {}",
            heading_sum
        );
        assert!(
            sum == expected_sum,
            "Expected {expected_sum:?}, got {}",
            sum
        );
        assert!(
            trima == expected_trima,
            "Expected {expected_trima:?}, got {}",
            trima
        );
    }

    #[test]
    fn next_trima_unchecked_odd_ok() {
        let period = 5;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let expected_sum = 36.0; // 2 + 3 + 3 + 4 + 4 + 4 + 5 + 5 + 6
        let expected_heading_sum = 11.0; // 5 + 6
        let expected_trailing_sum = 9.0; // 2 + 3 + 4
        let expected_trima = 4.0; // 36 / 9
        let (trima, sum, trailing_sum, heading_sum) =
            trima_next_odd_unchecked(6.0, 4.0, 1.0, 27.0, 6.0, 9.0, inv_weight_sum);

        assert!(
            trailing_sum == expected_trailing_sum,
            "Expected trailing_sum {expected_trailing_sum:?}, got {}",
            trailing_sum
        );
        assert!(
            heading_sum == expected_heading_sum,
            "Expected heading_sum {expected_heading_sum:?}, got {}",
            heading_sum
        );
        assert!(
            sum == expected_sum,
            "Expected sum {expected_sum:?}, got {}",
            sum
        );
        assert!(
            trima == expected_trima,
            "Expected {expected_trima:?}, got {}",
            trima
        );
    }

    #[test]
    fn next2_trima_unchecked_odd_ok() {
        let period = 5;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let expected_heading_sum = 13.0; // 6 + 7
        let expected_trailing_sum = 12.0; // 3 + 4 + 5
        let expected_sum = 45.0; // 3 + 4 + 4 + 5 + 5 + 5 + 6 + 6 + 7
        let expected_trima = 5.0; // 45 / 9
        let (trima, sum, trailing_sum, heading_sum) =
            trima_next_odd_unchecked(7.0, 5.0, 2.0, 36.0, 9.0, 11.0, inv_weight_sum);

        assert!(
            trailing_sum == expected_trailing_sum,
            "Expected trailing_sum {expected_trailing_sum:?}, got {}",
            trailing_sum
        );
        assert!(
            heading_sum == expected_heading_sum,
            "Expected heading_sum {expected_heading_sum:?}, got {}",
            heading_sum
        );
        assert!(
            sum == expected_sum,
            "Expected sum {expected_sum:?}, got {}",
            sum
        );
        assert!(
            trima == expected_trima,
            "Expected {expected_trima:?}, got {}",
            trima
        );
    }

    #[test]
    fn next3_trima_unchecked_odd_ok() {
        let period = 5;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let expected_heading_sum = 15.0; // 7 + 8
        let expected_trailing_sum = 15.0; // 4 + 5 + 6
        let expected_sum = 54.0; // 4 + 5 + 5 + 6 + 6 + 6 + 7 + 7 + 8
        let expected_trima = 6.0; // 54 / 9
        let (trima, sum, trailing_sum, heading_sum) =
            trima_next_odd_unchecked(8.0, 6.0, 3.0, 45.0, 12.0, 13.0, inv_weight_sum);

        assert!(
            trailing_sum == expected_trailing_sum,
            "Expected trailing_sum {expected_trailing_sum:?}, got {}",
            trailing_sum
        );
        assert!(
            heading_sum == expected_heading_sum,
            "Expected heading_sum {expected_heading_sum:?}, got {}",
            heading_sum
        );
        assert!(
            sum == expected_sum,
            "Expected sum {expected_sum:?}, got {}",
            sum
        );
        assert!(
            trima == expected_trima,
            "Expected {expected_trima:?}, got {}",
            trima
        );
    }

    #[test]
    fn next_trima_unchecked_even_ok() {
        let period = 6;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let (trima, sum, trailing_sum, heading_sum) =
            trima_next_even_unchecked(7.0, 4.0, 1.0, 42.0, 6.0, 15.0, inv_weight_sum);

        assert!(
            trailing_sum == 9.0,
            "Expected trailing_sum 9.0, got {}",
            trailing_sum
        );
        assert!(
            heading_sum == 18.0,
            "Expected heading_sum 18.0, got {}",
            heading_sum
        );
        assert!(sum == 54.0, "Expected sum 54.0, got {}", sum);
        assert!(trima == 4.5, "Expected 4.5, got {}", trima);
    }
}
