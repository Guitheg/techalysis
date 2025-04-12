#[derive(Debug)]
pub enum SmaError {
    OutOfRangeStartIndex,
    OutOfRangeEndIndex,
    BadParam(String),
    InsufficientData,
}

const MINIMAL_PERIOD_VALUE: usize = 2;
const MAXIMAL_PERIOD_VALUE: usize = 100_000;

pub fn lookback(period: usize) -> Result<usize, SmaError> {
    if period < MINIMAL_PERIOD_VALUE || period > MAXIMAL_PERIOD_VALUE {
        Err(SmaError::BadParam(format!(
            "period must be between {MINIMAL_PERIOD_VALUE} and {MAXIMAL_PERIOD_VALUE}"
        )))
    } else {
        Ok(period - 1)
    }
}

pub fn sma(
    start_idx: usize,
    end_idx: usize,
    data: &[f64],
    period: usize,
) -> Result<(usize, Vec<f64>), SmaError> {
    if start_idx > end_idx || end_idx >= data.len() {
        return Err(SmaError::OutOfRangeEndIndex);
    }

    let lookback_result = lookback(period)?;
    if data.len() < period {
        return Err(SmaError::InsufficientData);
    }

    let start_idx = if start_idx < lookback_result {
        lookback_result
    } else {
        start_idx
    };
    if start_idx > end_idx {
        return Ok((start_idx, Vec::new()));
    }

    let capacity = end_idx - start_idx + 1;
    let period_as_double = period as f64;
    let mut data_out = Vec::with_capacity(capacity);

    let mut trailing_idx = start_idx - lookback_result;
    let mut total_in_period = 0.0;

    if period > 1 {
        for i in trailing_idx..start_idx {
            total_in_period += data[i];
        }
    }

    for i in start_idx..=end_idx {
        total_in_period += data[i];
        let average = total_in_period / period_as_double;
        data_out.push(average);
        total_in_period -= data[trailing_idx];
        trailing_idx += 1;
    }

    Ok((start_idx, data_out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookback_failure_too_low() {
        let period: usize = 1;
        let result = lookback(period);
        assert!(result.is_err());
        if let Err(SmaError::BadParam(msg)) = result {
            assert!(msg.contains("between 2 and 100000"));
        }
    }

    #[test]
    fn lookback_failure_too_high() {
        let period: usize = 100_001;
        let result = lookback(period);
        assert!(result.is_err());
        if let Err(SmaError::BadParam(msg)) = result {
            assert!(msg.contains("between 2 and 100000"));
        }
    }

    #[test]
    fn lookback_success() {
        let period: usize = 30;
        let result = lookback(period);
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value == 29);
        }
    }

    #[test]
    fn test_sma_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let (out_beg_idx, result) = sma(0, data.len() - 1, &data, opt_in_time_period).unwrap();
        assert_eq!(out_beg_idx, 2);
        assert_eq!(result, vec![2.0, 3.0, 4.0, 4.0, 4.0, 3.0]);
    }

    #[test]
    fn test_invalid_period() {
        let data = vec![1.0, 2.0, 3.0];
        let result = sma(0, data.len() - 1, &data, 1);
        assert!(result.is_err());
        if let Err(SmaError::BadParam(msg)) = result {
            assert!(msg.contains("between 2 and 100000"));
        }
    }

    #[test]
    fn test_insufficient_data() {
        let data = vec![1.0, 2.0];
        let result = sma(0, data.len() - 1, &data, 3);
        assert!(matches!(result, Err(SmaError::InsufficientData)));
    }
}
