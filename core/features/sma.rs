use std::f64;

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

pub fn sma(data_array: &[f64], period: usize) -> Result<Vec<f64>, SmaError> {
    let lookback_result = lookback(period)?;
    if data_array.len() < period {
        return Err(SmaError::InsufficientData);
    }

    let start_idx = if lookback_result > 0 {
        lookback_result
    } else {
        0
    };

    let period_as_double = period as f64;
    let mut data_out = Vec::with_capacity(data_array.len());

    let mut trailing_idx = start_idx - lookback_result;
    let mut total_in_period = 0.0;

    if period > 1 {
        for data in data_array[trailing_idx..start_idx].iter() {
            total_in_period += data;
            data_out.push(f64::NAN);
        }
    }

    for data in data_array[start_idx..].iter() {
        total_in_period += data;
        data_out.push(total_in_period / period_as_double);

        total_in_period -= data_array[trailing_idx];
        trailing_idx += 1;
    }

    Ok(data_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_vec_float_eq {
        ($a:expr, $b:expr) => {{
            for (i, (x, y)) in $a.iter().zip($b.iter()).enumerate() {
                if x.is_nan() && y.is_nan() {
                    continue;
                }
                assert!(
                    (x - y).abs() < std::f64::EPSILON,
                    "Failed at index {} -> {} != {}",
                    i,
                    x,
                    y
                );
            }
        }};
    }

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
        let result = sma(&data, opt_in_time_period).unwrap();
        assert_vec_float_eq!(
            result,
            vec![f64::NAN, f64::NAN, 2.0, 3.0, 4.0, 4.0, 4.0, 3.0]
        );
    }

    #[test]
    fn test_invalid_period() {
        let data = vec![1.0, 2.0, 3.0];
        let result = sma(&data, 1);
        assert!(result.is_err());
        if let Err(SmaError::BadParam(msg)) = result {
            assert!(msg.contains("between 2 and 100000"));
        }
    }

    #[test]
    fn test_insufficient_data() {
        let data = vec![1.0, 2.0];
        let result = sma(&data, 3);
        assert!(matches!(result, Err(SmaError::InsufficientData)));
    }
}
