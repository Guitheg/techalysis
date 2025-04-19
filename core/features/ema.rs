use crate::helper::{errors::TechnicalysisError, loopback::lookback};

use super::sma::sma;

pub fn ema(
    data_array: &[f64],
    window_size: usize,
    smoothing_factor: f64,
) -> Result<Vec<f64>, TechnicalysisError> {
    let lookback_result = lookback(window_size)?;
    if data_array.len() < window_size {
        return Err(TechnicalysisError::InsufficientData);
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
    let mut good_counter: usize = window_size;
    let multiplier = smoothing_factor / (period_as_double + 1f64);

    let sma_result = sma(&data_array[trailing_idx..start_idx + 1], window_size)?;
    data_out.extend_from_slice(&sma_result);

    for data in data_array[start_idx + 1..].iter() {
        if (*data).is_nan() {
            nan_counter = window_size;
            good_counter = 0;
            total_in_period = 0.0;
        } else {
            good_counter += 1;
            total_in_period += data;
        }
        let trailing_value = data_array[trailing_idx];
        if nan_counter == 0 {
            if let Some(previous) = data_out.last() {
                if good_counter == window_size {
                    data_out.push(total_in_period / period_as_double)
                } else {
                    data_out.push((data * multiplier) + (previous * (1f64 - multiplier)));
                }
            }

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
    use crate::{assert_vec_float_eq, features::ema::ema};

    #[test]
    fn test_ema_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = ema(&data, opt_in_time_period, 2f64).unwrap();
        assert_vec_float_eq!(
            result,
            vec![f64::NAN, f64::NAN, 2.0, 3.0, 4.0, 3.5, 3.75, 2.875],
            1e-6
        );
    }

    #[test]
    fn test_ema_bigger() {
        let data = vec![
            0.748179, 0.229069, 0.791048, 0.095412, 0.545816, 0.330744, 0.197317, 0.336571,
            0.139322, 0.282722, 0.278997, 0.394725, 0.682038, 0.139297, 0.701122, 0.278533,
            0.499061, 0.526171, 0.778530, 0.672986, 0.026303, 0.140771, 0.211985, 0.639840,
            0.505245, 0.401956, 0.572973, 0.574235, 0.814633, 0.991350, 0.536899, 0.791905,
            0.002872, 0.797976, 0.814290, 0.084729, 0.508093, 0.324266, 0.610305, 0.946679,
            0.375828, 0.251873, 0.817190, 0.844160, 0.656774, 0.387461, 0.402088, 0.494828,
            0.018140, 0.268551,
        ];
        let opt_in_time_period = 10;
        let result = ema(&data, opt_in_time_period, 2f64).unwrap();
        assert_vec_float_eq!(
            result,
            vec![
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                0.369620,
                0.353143,
                0.360704,
                0.419128,
                0.368250,
                0.428772,
                0.401456,
                0.419202,
                0.438651,
                0.500447,
                0.531818,
                0.439906,
                0.385518,
                0.353966,
                0.405943,
                0.423998,
                0.419991,
                0.447806,
                0.470793,
                0.533309,
                0.616589,
                0.602100,
                0.636610,
                0.521385,
                0.571674,
                0.615786,
                0.519230,
                0.517206,
                0.482126,
                0.505431,
                0.585658,
                0.547507,
                0.493755,
                0.552562,
                0.605580,
                0.614888,
                0.573537,
                0.542365,
                0.533722,
                0.439980,
                0.408811
            ],
            1e-6
        );
    }

    #[test]
    fn test_ema_with_nan() {
        let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = ema(&data, opt_in_time_period, 2f64).unwrap();
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
            ],
            1e-6
        );
    }
}
