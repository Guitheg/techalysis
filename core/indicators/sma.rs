use crate::errors::TechnicalysisError;
use std::f64;

pub fn sma(data_array: &[f64], period: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();
    let mut output = vec![f64::NAN; size];
    sma_into(data_array, period, &mut output)?;
    Ok(output)
}

pub fn sma_into(
    data_array: &[f64],
    period: usize,
    output: &mut [f64],
) -> Result<(), TechnicalysisError> {
    let size = data_array.len();
    if period == 0 || period > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechnicalysisError::BadParam(
            "SMA period must be greater than 1".to_string(),
        ));
    }

    let mut running_sum: f64 = 0.0;
    for idx in 0..period {
        let value = &data_array[idx];
        if value.is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        } else {
            running_sum += value;
        }
        output[idx] = f64::NAN;
    }
    output[period - 1] = running_sum / period as f64;

    for idx in period..size {
        if data_array[idx].is_nan() {
            return Err(TechnicalysisError::UnexpectedNan);
        }
        running_sum += data_array[idx] - data_array[idx - period];
        output[idx] = running_sum / period as f64;
    }
    Ok(())
}
