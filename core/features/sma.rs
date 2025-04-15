use std::f64;

#[derive(Debug)]
pub enum SmaError {
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

pub fn sma(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, SmaError> {
    let lookback_result = lookback(window_size)?;
    if data_array.len() < window_size {
        return Err(SmaError::InsufficientData);
    }

    let start_idx = if lookback_result > 0 {
        lookback_result
    } else {
        0
    };

    let period_as_double = window_size as f64;
    let mut data_out = Vec::with_capacity(data_array.len());

    let mut trailing_idx = start_idx - lookback_result;
    let mut total_in_period = 0.0;
    let mut nan_counter = 0;

    if window_size > 1 {
        for data in data_array[trailing_idx..start_idx].iter() {
            if (*data).is_nan() {
                nan_counter = window_size;
                total_in_period = 0.0;
            } else {
                total_in_period += data;
            }

            if nan_counter != 0 {
                nan_counter = nan_counter.saturating_sub(1);
            }
            data_out.push(f64::NAN);
        }
    }

    for data in data_array[start_idx..].iter() {
        if (*data).is_nan() {
            nan_counter = window_size;
            total_in_period = 0.0;
        } else {
            total_in_period += data;
        }
        let trailing_value = data_array[trailing_idx];
        if nan_counter == 0 {
            data_out.push(total_in_period / period_as_double);
            if !trailing_value.is_nan() {
                total_in_period -= trailing_value;
            }
        } else {
            data_out.push(f64::NAN);
            nan_counter = nan_counter.saturating_sub(1);
        }
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
    fn test_sma_nan_in_begin() {
        let data = vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period).unwrap();
        assert_vec_float_eq!(
            result,
            vec![f64::NAN, f64::NAN, f64::NAN, f64::NAN, 4.0, 4.0, 4.0, 3.0]
        );
    }

    #[test]
    fn test_sma_with_nan() {
        let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period).unwrap();
        assert_vec_float_eq!(
            result,
            vec![
                f64::NAN,
                f64::NAN,
                2.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                4.0,
                3.0
            ]
        );
    }

    #[test]
    fn test_sma_with_two_nans() {
        let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, f64::NAN, 2.0, 3.0, 4.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period).unwrap();
        assert_vec_float_eq!(
            result,
            vec![
                f64::NAN,
                f64::NAN,
                2.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                3.0
            ]
        );
    }

    #[test]
    fn test_sma_with_three_nans() {
        let data = vec![
            1.0,
            2.0,
            3.0,
            f64::NAN,
            4.0,
            f64::NAN,
            2.0,
            3.0,
            4.0,
            f64::NAN,
            2.0,
            3.0,
            4.0,
        ];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period).unwrap();
        assert_vec_float_eq!(
            result,
            vec![
                f64::NAN,
                f64::NAN,
                2.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                3.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                3.0
            ]
        );
    }

    #[test]
    fn test_sma_with_more_nans() {
        let data = vec![
            1.0,
            2.0,
            3.0,
            4.0,
            5.0,
            6.0,
            7.0,
            8.0,
            f64::NAN,
            4.0,
            f64::NAN,
            2.0,
            3.0,
            4.0,
            f64::NAN,
            2.0,
            3.0,
            4.0,
        ];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period).unwrap();
        assert_vec_float_eq!(
            result,
            vec![
                f64::NAN,
                f64::NAN,
                2.0,
                3.0,
                4.0,
                5.0,
                6.0,
                7.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                3.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                3.0
            ]
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
