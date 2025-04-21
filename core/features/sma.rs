use crate::errors::TechnicalysisError;
use std::f64;

pub fn sma(
    data_array: &[f64],
    window_size: usize,
    handle_nan: bool,
) -> Result<Vec<f64>, TechnicalysisError> {
    if handle_nan {
        sma_nan_safe(data_array, window_size)
    } else {
        sma_raw(data_array, window_size)
    }
}

fn sma_raw(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();
    if window_size == 0 || window_size > size {
        return Err(TechnicalysisError::InsufficientData);
    }

    if window_size == 1 {
        return Ok(data_array.to_vec());
    }

    let mut result = vec![f64::NAN; size];

    let mut running_sum: f64 = data_array[..window_size].iter().sum();
    result[window_size - 1] = running_sum / window_size as f64;

    for i in window_size..size {
        running_sum += data_array[i] - data_array[i - window_size];
        result[i] = running_sum / window_size as f64;
    }
    Ok(result)
}

#[inline]
fn sma_nan_safe(data_array: &[f64], window_size: usize) -> Result<Vec<f64>, TechnicalysisError> {
    let size = data_array.len();

    if window_size == 0 || window_size > size {
        return Err(TechnicalysisError::InsufficientData);
    }
    if window_size == 1 {
        return Ok(data_array.to_vec());
    }

    let mut result = vec![f64::NAN; size];

    let mut running_sum = 0.0;
    let mut nan_count = 0usize;

    for &v in &data_array[..window_size] {
        if v.is_nan() {
            nan_count += 1;
        } else {
            running_sum += v;
        }
    }

    if nan_count == 0 {
        result[window_size - 1] = running_sum / window_size as f64;
    }

    for i in window_size..size {
        let old = data_array[i - window_size];
        if old.is_nan() {
            nan_count -= 1;
        } else {
            running_sum -= old;
        }

        let new = data_array[i];
        if new.is_nan() {
            nan_count += 1;
        } else {
            running_sum += new;
        }

        if nan_count == 0 {
            result[i] = running_sum / window_size as f64;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::assert_vec_float_eq;
    use crate::errors::TechnicalysisError;

    use super::*;

    #[test]
    fn test_sma_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period, false).unwrap();
        assert_vec_float_eq!(
            result,
            vec![f64::NAN, f64::NAN, 2.0, 3.0, 4.0, 4.0, 4.0, 3.0],
            f64::EPSILON
        );
    }

    #[test]
    fn test_sma_bigger() {
        let data = vec![
            0.63353828, 0.7651939, 0.61698533, 0.85637095, 0.04992363, 0.80417807, 0.68243138,
            0.48786552, 0.58232622, 0.69049764, 0.28500753, 0.84544206, 0.55750837, 0.59586001,
            0.94631594, 0.71311121, 0.87440502, 0.57908696, 0.11535073, 0.16831031, 0.23428027,
            0.15277727, 0.87060905, 0.45383095, 0.38579641, 0.68332434, 0.98559273, 0.15593885,
            0.72471422, 0.70597371, 0.4645111, 0.8905361, 0.50290464, 0.14588416, 0.06937085,
            0.70680851, 0.3512258, 0.04145161, 0.43660052, 0.08932176, 0.44373486, 0.92205579,
            0.09826058, 0.76490888, 0.01060093, 0.94912147, 0.76627469, 0.50807698, 0.56629695,
            0.89206963, 0.66770969, 0.94152494, 0.16219879, 0.31110991, 0.02140634, 0.14895287,
            0.26307974, 0.43348417, 0.56694172, 0.74949139, 0.57558831, 0.29276144, 0.92369964,
            0.03149536, 0.32154615, 0.08113183, 0.37958128, 0.74585891, 0.40677693, 0.12934919,
            0.75451143, 0.19895636, 0.35328043, 0.70899457, 0.75235621, 0.74939298, 0.95536452,
            0.18762026, 0.57533369, 0.3725725, 0.54545985, 0.77246798, 0.10215389, 0.55523726,
            0.16977556, 0.1936754, 0.84627931, 0.30930139, 0.65903575, 0.8439534, 0.55970775,
            0.77278444, 0.06343206, 0.38075072, 0.85839897, 0.14769779, 0.85710671, 0.28479114,
            0.21016676, 0.66442761,
        ];
        let opt_in_time_period = 20;
        let result = sma(&data, opt_in_time_period, true).unwrap();
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
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                0.59248545,
                0.57252255,
                0.54190172,
                0.55458291,
                0.53445591,
                0.55124955,
                0.54520686,
                0.56036493,
                0.54376859,
                0.55088799,
                0.5516618,
                0.56063697,
                0.56289168,
                0.56016149,
                0.5376627,
                0.49381544,
                0.49350031,
                0.46734135,
                0.44045958,
                0.45652207,
                0.45257264,
                0.46304537,
                0.5015093,
                0.46289187,
                0.47844577,
                0.459686,
                0.47297585,
                0.46200995,
                0.47961686,
                0.47169599,
                0.48100079,
                0.49116072,
                0.49371016,
                0.47667487,
                0.48493616,
                0.48253793,
                0.45464515,
                0.45023785,
                0.46983948,
                0.47635654,
                0.50936502,
                0.51595769,
                0.48449297,
                0.52576493,
                0.48909425,
                0.50464151,
                0.46124203,
                0.44190736,
                0.45379645,
                0.44582045,
                0.40768443,
                0.41202452,
                0.37489609,
                0.38445017,
                0.4043444,
                0.4408919,
                0.4709139,
                0.50552814,
                0.49323495,
                0.49365455,
                0.4748086,
                0.47330218,
                0.4972875,
                0.45621022,
                0.48239731,
                0.47480878,
                0.48043596,
                0.50377086,
                0.48194299,
                0.49455593,
                0.53028614,
                0.52054595,
                0.54923736,
                0.53474494,
                0.51833275,
                0.52363488,
                0.49355013,
                0.48863724,
                0.49349578,
                0.47523743,
                0.48983019
            ],
            1e-8
        );
    }

    #[test]
    fn test_sma_nan_in_begin() {
        let data = vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period, true).unwrap();
        assert_vec_float_eq!(
            result,
            vec![f64::NAN, f64::NAN, f64::NAN, f64::NAN, 4.0, 4.0, 4.0, 3.0],
            f64::EPSILON
        );
    }

    #[test]
    fn test_sma_with_nan() {
        let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, 4.0, 2.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period, true).unwrap();
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
            f64::EPSILON
        );
    }

    #[test]
    fn test_sma_with_two_nans() {
        let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, f64::NAN, 2.0, 3.0, 4.0];
        let opt_in_time_period = 3;
        let result = sma(&data, opt_in_time_period, true).unwrap();
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
            ],
            f64::EPSILON
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
        let result = sma(&data, opt_in_time_period, true).unwrap();
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
            ],
            f64::EPSILON
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
        let result = sma(&data, opt_in_time_period, true).unwrap();
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
            ],
            f64::EPSILON
        );
    }

    #[test]
    fn test_invalid_period() {
        let data = vec![1.0, 2.0, 3.0];
        let result = sma(&data, 0, true);
        assert!(result.is_err());
        if let Err(TechnicalysisError::BadParam(msg)) = result {
            assert!(msg.contains("between 2 and 100000"));
        }
    }

    #[test]
    fn test_insufficient_data() {
        let data = vec![1.0, 2.0];
        let result = sma(&data, 3, true);
        assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
    }
}
