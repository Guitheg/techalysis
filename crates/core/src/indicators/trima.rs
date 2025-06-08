use std::collections::VecDeque;

use crate::errors::TechalysisError;
use crate::types::Float;

#[derive(Debug)]
pub struct TrimaResult {
    pub values: Vec<Float>,
    pub state: TrimaState,
}

#[derive(Debug, Clone)]
pub struct TrimaState {
    pub trima: Float,
    pub sum: Float,
    pub trailing_sum: Float,
    pub heading_sum: Float,
    pub window: VecDeque<Float>,
    pub inv_weight_sum: Float,
    pub period: usize,

}

impl From<TrimaResult> for Vec<Float> {
    fn from(result: TrimaResult) -> Self {
        result.values
    }
}

impl TrimaState {
    pub fn next(&self, new_value: Float) -> Result<TrimaState, TechalysisError> {
        trima_next(
            new_value,
            self
        )
    }
}

pub fn trima_next(
    new_value: Float,
    state: &TrimaState,
) -> Result<TrimaState, TechalysisError> {
    if state.period <= 1 {
        return Err(TechalysisError::BadParam(
            "TRIMA period must be greater than 1".to_string(),
        ));
    }
    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}"
        )));
    }
    if !state.trima.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_trima = {:?}", state.trima
        )));
    }
    if state.window.len() != state.period {
        return Err(TechalysisError::BadParam(
            "Window length must match the TRIMA period".to_string(),
        ));
    }

    for (idx, &value) in state.window.iter().enumerate() {
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "window[{idx}] = {value:?}"
            )));
        }
    }
    let is_odd = state.period % 2 != 0;

    let mut window = state.window.clone();

    let old_value = window
        .pop_front()
        .ok_or(TechalysisError::InsufficientData)?;
    window.push_back(new_value);
    let vec = Vec::from(window.clone());
    let middle_idx = get_middle_idx(state.period);
    let middle_value = vec[middle_idx];


    let (trima, new_sum, new_trailing_sum, new_heading_sum) = if is_odd {
        trima_next_odd_unchecked(
            new_value,
            middle_value,
            old_value,
            state.sum,
            state.trailing_sum,
            state.heading_sum,
            state.inv_weight_sum,
        )
    } else {
        trima_next_even_unchecked(
            new_value,
            middle_value,
            old_value,
            state.sum,
            state.trailing_sum,
            state.heading_sum,
            state.inv_weight_sum,
        )
    };

    if !trima.is_finite() {
        return Err(TechalysisError::Overflow(0, trima));
    }

    Ok(TrimaState {
        trima,
        sum: new_sum,
        trailing_sum: new_trailing_sum,
        heading_sum: new_heading_sum,
        window,
        inv_weight_sum: state.inv_weight_sum,
        period: state.period,
    })
}

pub fn trima(data: &[Float], period: usize) -> Result<TrimaResult, TechalysisError> {
    let len = data.len();
    let mut output = vec![0.0; len];
    let trima_state = trima_into(data, period, &mut output)?;
    Ok(TrimaResult {
        values: output,
        state: trima_state,
    })
}

pub fn trima_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<TrimaState, TechalysisError> {
    let len = data.len();
    let is_odd = period % 2 != 0;
    if period == 0 || period > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "TRIMA period must be greater than 1".to_string(),
        ));
    }

    if output.len() < len {
        return Err(TechalysisError::BadParam(
            "Output array must be at least as long as the input data array".to_string(),
        ));
    }
    
    let (trima, mut sum, mut trailing_sum, mut heading_sum, inv_weight_sum, mut middle_idx) =
        init_trima_unchecked(data, period, output)?;

    output[period - 1] = trima;
    if !output[period - 1].is_finite() {
        return Err(TechalysisError::Overflow(period - 1, output[period - 1]));
    }
    middle_idx += 1;
    
    if is_odd {    
        for idx in period..len {
            if !data[idx].is_finite() {
                return Err(TechalysisError::DataNonFinite(format!(
                    "data[{idx}] = {:?}",
                    data[idx]
                )));
            }
            (output[idx], sum, trailing_sum, heading_sum) = trima_next_odd_unchecked(
                data[idx],
                data[middle_idx],
                data[idx - period],
                sum,
                trailing_sum,
                heading_sum,
                inv_weight_sum,
            );
            if !output[idx].is_finite() {
                return Err(TechalysisError::Overflow(idx, output[idx]));
            }
            middle_idx += 1;
        }
    } else {
        for idx in period..len {
            if !data[idx].is_finite() {
                return Err(TechalysisError::DataNonFinite(format!(
                    "data[{idx}] = {:?}",
                    data[idx]
                )));
            }
            (output[idx], sum, trailing_sum, heading_sum) = trima_next_even_unchecked(
                data[idx],
                data[middle_idx],
                data[idx - period],
                sum,
                trailing_sum,
                heading_sum,
                inv_weight_sum,
            );
            if !output[idx].is_finite() {
                return Err(TechalysisError::Overflow(idx, output[idx]));
            }
            middle_idx += 1;
        }
    }
    
    Ok(TrimaState {
        trima: output[len - 1],
        sum,
        trailing_sum,
        heading_sum,
        window: VecDeque::from(data[len - period..len].to_vec()),
        inv_weight_sum,
        period,
    })
}

#[inline(always)]
pub fn trima_next_even_unchecked(
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
pub fn trima_next_odd_unchecked(
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
) -> Result<(Float, Float, Float, Float, Float, usize), TechalysisError> {
    let middle_idx = get_middle_idx(period);
    let mut trailing_sum: Float = 0.0;
    let mut heading_sum: Float = 0.0;
    let mut sum: Float = 0.0;
    let inv_weight_sum = trima_inv_weight_sum(period)?;
    for idx in 0..=middle_idx {
        let weight = (idx + 1) as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data_array[{idx}] = {value:?}"
            )));
        }
        trailing_sum += value;
        sum += value * weight;
        output[idx] = Float::NAN;
    }
    for (local_idx, idx) in (middle_idx+1..period).rev().enumerate() {
        let weight = (local_idx + 1) as Float;
        let value = &data[idx];
        if !value.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
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

fn trima_inv_weight_sum(period: usize) -> Result<Float, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
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
        let expected_sum: f64 = 27.0; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 + 5) = 27
        let expected_trailing_sum = 6.0; // (1 + 2 + 3) = 6, but we only take the first half
        let expected_heading_sum = 9.0; // (4 + 5) = 12, but we only take the second half
        let expected_middle_idx = 2; // middle index for period 5 is 2
        let expected_trima = 3.0; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 + 5) * (1/9) = 3.0
        
        let (trima, sum, trailing_sum, heading_sum, inv_weight_sum , middle_idx) = init_trima_unchecked(&data, period, &mut output).unwrap();

        assert!(output.iter().take(period-1).all(|&v| v.is_nan()), "Expected first {} values to be NaN", period - 1);
        assert!(inv_weight_sum == expected_inv_weight_sum, "Expected inv_weight_sum to be {}, got {}", expected_inv_weight_sum, inv_weight_sum);
        assert!(middle_idx == expected_middle_idx, "Expected middle_idx to be {expected_middle_idx:?}, got {}", middle_idx);
        assert!(trailing_sum == expected_trailing_sum, "Expected {expected_trailing_sum:?}, got {}", trailing_sum);
        assert!(heading_sum == expected_heading_sum, "Expected {expected_heading_sum:?}, got {}", heading_sum);
        assert!(sum == expected_sum, "Expected {expected_sum:?}, got {}", sum);
        assert!(trima == expected_trima, "Expected trima {expected_trima:?}, got {}", trima);
    }

    #[test]
    fn init_trima_unchecked_even_ok() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let period = 6;
        let mut output = vec![0.0; data.len()];
        let expected_inv_weight_sum = 1.0 / 12.0; // (6//2) * (6//2 + 1) = 12, so inv_weight_sum = 1/12
        let expected_sum: f64 = 42.0; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 +4 + 5 + 5 + 6) = 42
        let expected_trailing_sum = 6.0; // (1 + 2 + 3) = 6, but we only take the first half
        let expected_heading_sum = 15.0; // (4 + 5 + 6) = 15, but we only take the second half
        let expected_middle_idx = 2; // middle index for period 6 is 3
        let expected_trima = 3.5; // (1 + 2 + 2 + 3 + 3 + 3 + 4 + 4 + 4 + 5 + 5 + 6) * (1/12) = 3.5
        
        let (trima, sum, trailing_sum, heading_sum, inv_weight_sum , middle_idx) = init_trima_unchecked(&data, period, &mut output).unwrap();

        assert!(output.iter().take(period-1).all(|&v| v.is_nan()), "Expected first {} values to be NaN", period - 1);
        assert!(inv_weight_sum == expected_inv_weight_sum, "Expected inv_weight_sum to be {}, got {}", expected_inv_weight_sum, inv_weight_sum);
        assert!(middle_idx == expected_middle_idx, "Expected middle_idx to be {expected_middle_idx:?}, got {}", middle_idx);
        assert!(trailing_sum == expected_trailing_sum, "Expected {expected_trailing_sum:?}, got {}", trailing_sum);
        assert!(heading_sum == expected_heading_sum, "Expected {expected_heading_sum:?}, got {}", heading_sum);
        assert!(sum == expected_sum, "Expected {expected_sum:?}, got {}", sum);
        assert!(trima == expected_trima, "Expected {expected_trima:?}, got {}", trima);
    }

    #[test]
    fn next_trima_unchecked_odd_ok() {
        let period = 5;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let expected_sum = 36.0; // 2 + 3 + 3 + 4 + 4 + 4 + 5 + 5 + 6
        let expected_heading_sum = 11.0; // 5 + 6
        let expected_trailing_sum = 9.0; // 2 + 3 + 4
        let expected_trima = 4.0; // 36 / 9
        let (trima, sum, trailing_sum, heading_sum) = trima_next_odd_unchecked(6.0, 4.0, 1.0, 27.0, 6.0, 9.0, inv_weight_sum);

        assert!(trailing_sum == expected_trailing_sum, "Expected trailing_sum {expected_trailing_sum:?}, got {}", trailing_sum);
        assert!(heading_sum == expected_heading_sum, "Expected heading_sum {expected_heading_sum:?}, got {}", heading_sum);
        assert!(sum == expected_sum, "Expected sum {expected_sum:?}, got {}", sum);
        assert!(trima == expected_trima, "Expected {expected_trima:?}, got {}", trima);
    }

    #[test]
    fn next2_trima_unchecked_odd_ok() {
        let period = 5;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let expected_heading_sum = 13.0; // 6 + 7
        let expected_trailing_sum = 12.0; // 3 + 4 + 5
        let expected_sum = 45.0; // 3 + 4 + 4 + 5 + 5 + 5 + 6 + 6 + 7
        let expected_trima = 5.0; // 45 / 9
        let (trima, sum, trailing_sum, heading_sum) = trima_next_odd_unchecked(7.0, 5.0, 2.0, 36.0, 9.0, 11.0, inv_weight_sum);

        assert!(trailing_sum == expected_trailing_sum, "Expected trailing_sum {expected_trailing_sum:?}, got {}", trailing_sum);
        assert!(heading_sum == expected_heading_sum, "Expected heading_sum {expected_heading_sum:?}, got {}", heading_sum);
        assert!(sum == expected_sum, "Expected sum {expected_sum:?}, got {}", sum);
        assert!(trima == expected_trima, "Expected {expected_trima:?}, got {}", trima);
    }

    #[test]
    fn next3_trima_unchecked_odd_ok() {
        let period = 5;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let expected_heading_sum = 15.0; // 7 + 8
        let expected_trailing_sum = 15.0; // 4 + 5 + 6
        let expected_sum = 54.0; // 4 + 5 + 5 + 6 + 6 + 6 + 7 + 7 + 8
        let expected_trima = 6.0; // 54 / 9
        let (trima, sum, trailing_sum, heading_sum) = trima_next_odd_unchecked(8.0, 6.0, 3.0, 45.0, 12.0, 13.0, inv_weight_sum);

        assert!(trailing_sum == expected_trailing_sum, "Expected trailing_sum {expected_trailing_sum:?}, got {}", trailing_sum);
        assert!(heading_sum == expected_heading_sum, "Expected heading_sum {expected_heading_sum:?}, got {}", heading_sum);
        assert!(sum == expected_sum, "Expected sum {expected_sum:?}, got {}", sum);
        assert!(trima == expected_trima, "Expected {expected_trima:?}, got {}", trima);
    }


    #[test]
    fn next_trima_unchecked_even_ok() {
        let period = 6;
        let inv_weight_sum = trima_inv_weight_sum(period).unwrap();
        let (trima, sum, trailing_sum, heading_sum) = trima_next_even_unchecked(7.0, 4.0, 1.0, 42.0, 6.0, 15.0, inv_weight_sum);

        assert!(trailing_sum == 9.0, "Expected trailing_sum 9.0, got {}", trailing_sum);
        assert!(heading_sum == 18.0, "Expected heading_sum 18.0, got {}", heading_sum);
        assert!(sum == 54.0, "Expected sum 54.0, got {}", sum);
        assert!(trima == 4.5, "Expected 4.5, got {}", trima);
    }
}