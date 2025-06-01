use crate::assert_vec_float_eq;
use crate::rust::tests_helper::assert::assert_vec_close;
use crate::rust::tests_helper::generated::{assert_vec_eq_gen_data, load_generated_csv};
use proptest::{prop_assert, prop_assert_eq, proptest};
use technicalysis::errors::TechnicalysisError;
use technicalysis::indicators::rsi;

#[test]
fn generated() {
    let columns = load_generated_csv("rsi.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = rsi(input, 14);
    assert!(output.is_ok());
    let out = output.unwrap();
    let expected = columns.get("out").unwrap();
    assert_vec_eq_gen_data(&out, expected);
    assert!(out.len() == input.len());
}

#[test]
fn empty_input() {
    let data: [f64; 0] = [];
    let period = 14;
    let result = rsi(&data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

#[test]
fn input_shorter_than_period() {
    let data = &[1.0, 2.0, 3.0];
    let period = 5;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

#[test]
fn input_length_equals_period() {
    let data = &[1.0, 2.0, 3.0];
    let period = 3;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

#[test]
fn period_1() {
    let data = &[10.0, 11.0, 10.0, 10.0, 12.0];
    let period = 1;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::BadParam(_))));
}

#[test]
fn min_data_for_one_value_mixed() {
    let data = &[10.0, 11.0, 10.0];
    let period = 2;
    let expected = vec![f64::NAN, f64::NAN, 50.0];
    let result = rsi(data, period);
    assert_vec_close(&result.unwrap(), &expected);
}

#[test]
fn alternating_up_down() {
    let data = &[10.0, 12.0, 10.0, 12.0, 10.0, 12.0];
    let period = 2;
    let expected = vec![f64::NAN, f64::NAN, 50.0, 75.0, 37.5, 68.75];
    let result = rsi(data, period);
    assert_vec_close(&result.unwrap(), &expected);
}

#[test]
fn all_values_approximatelly_same() {
    let data = &[
        10.1, 9.9, 10.0, 10.2, 10.1, 10.12, 10.11, 10.12, 10.11, 10.10, 10.09, 10.10,
    ];
    let period = 3;
    let expected = vec![
        f64::NAN,
        f64::NAN,
        f64::NAN,
        60.0,
        46.15384615,
        49.64028777,
        47.34133791,
        50.76182839,
        46.25502375,
        40.81895857,
        34.70156925,
        46.68643523,
    ];
    let result = rsi(data, period);
    assert_vec_float_eq!(&result.unwrap(), &expected, 1e-6);
}

#[test]
fn all_values_same() {
    let data = &[10.0, 10.0, 10.0, 10.0, 10.0];
    let period = 3;
    let expected = vec![f64::NAN, f64::NAN, f64::NAN, 50.0, 50.0];
    let result = rsi(data, period);
    assert_vec_close(&result.unwrap(), &expected);
}

#[test]
fn all_increasing() {
    let data = &[1.0, 2.0, 3.0, 4.0, 5.0];
    let period = 3;
    let expected = vec![f64::NAN, f64::NAN, f64::NAN, 100.0, 100.0];
    let result = rsi(data, period);
    assert_vec_close(&result.unwrap(), &expected);
}

#[test]
fn all_decreasing() {
    let data = &[5.0, 4.0, 3.0, 2.0, 1.0];
    let period = 3;
    let expected = vec![f64::NAN, f64::NAN, f64::NAN, 0.0, 0.0];
    let result = rsi(data, period);
    assert_vec_close(&result.unwrap(), &expected);
}

#[test]
fn input_with_nans() {
    let data = &[1.0, 2.0, f64::NAN, 4.0, 5.0];
    let period = 2;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::UnexpectedNan)));
}

#[test]
fn period_zero() {
    let data = &[1.0, 2.0, 3.0];
    let period = 0;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

proptest! {
    #[test]
    fn proptest(
        data in proptest::collection::vec(-1000.0..1000.0, 1..100),
        period in 1..50
) {
        let period = period as usize;
        let result = rsi(&data, period);

        if data.len() <= period || period <= 1 {
            prop_assert!(result.is_err());
            if period <= 1 && data.len() > 1 {
                prop_assert!(matches!(result, Err(TechnicalysisError::BadParam(_))));
            } else {
                prop_assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
            }
        } else {
            let rsi_values = result.unwrap();
            prop_assert_eq!(rsi_values.len(), data.len());

            for value in rsi_values.iter().take(period) {
                prop_assert!(&value.is_nan());
            }

            for &value in &rsi_values[period..] {
                prop_assert!((0.0..=100.0).contains(&value));
            }
        }
    }
}
