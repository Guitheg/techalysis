use crate::oracle_test;
use crate::rust::tests_helper::assert::approx_eq_f64;
use crate::rust::tests_helper::{assert::assert_vec_close, oracle::read_fixture};
use proptest::{collection::vec, prelude::*};
use technicalysis::errors::TechnicalysisError;
use technicalysis::indicators::sma;

oracle_test!(sma, |x: &[f64]| sma(x, 30));

#[test]
fn test_length_preserved() {
    let (input, _) = read_fixture("oracle/sma");
    let output = sma(&input, 30);
    assert!(output.is_ok());
    let out = output.unwrap();
    assert_eq!(out.len(), input.len());
    assert!(out[..(30 - 1)].iter().all(|v| v.is_nan()));
}

#[test]
fn test_identity() {
    let (input, _) = read_fixture("oracle/sma");
    let output = sma(&input, 1);
    assert!(output.is_ok());
    let out = output.unwrap();
    assert_vec_close(&out, &input);
}

#[test]
fn test_linearity() {
    let (input, expected) = read_fixture("oracle/sma");
    const K: f64 = 9.3;
    let scaled_input: Vec<f64> = input.iter().map(|v| v * K).collect();
    let scaled_expected: Vec<f64> = expected.iter().map(|v| v * K).collect();
    let output = sma(&scaled_input, 30);
    assert!(output.is_ok());
    let out = output.unwrap();
    assert_vec_close(&out, &scaled_expected);
}

#[test]
fn test_extremum_value_injection_without_panic() {
    use std::f64;
    let data = vec![f64::MAX / 2.0, f64::MAX / 2.0, f64::MIN_POSITIVE, -0.0, 0.0];
    let out = sma(&data, 2).expect("sma must not error on finite extremes");
    assert_eq!(out.len(), data.len());
    for (i, v) in out.iter().enumerate() {
        if i < 1 {
            assert!(v.is_nan());
        } else {
            assert!(v.is_finite(), "value at {i} is not finite: {v}");
        }
    }
}

#[test]
fn test_invalid_period_lower_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 0);
    assert!(result.is_err());
    if let Err(TechnicalysisError::BadParam(msg)) = result {
        assert!(msg.contains("between 2 and 100000"));
    }
}

#[test]
fn test_period_higher_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 3);
    assert!(result.is_ok());
    let out = result.unwrap();
    assert!(out[2] == 2.0);
}

#[test]
fn test_unexpected_nan() {
    let data = vec![1.0, 2.0, 3.0, f64::NAN];
    let result = sma(&data, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::UnexpectedNan)));
}

#[test]
fn test_insufficient_data() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 4);
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

fn slow_sma(data: &[f64], window: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if window == 0 || window > data.len() {
        return out;
    }

    for i in window - 1..data.len() {
        let slice = &data[i + 1 - window..=i];
        out[i] = slice.iter().sum::<f64>() / window as f64;
    }
    out
}

proptest! {
    #[test]
    fn proptest_sma(
        input  in vec(-1e12f64..1e12, 2..200),
        window in 2usize..50
    ) {
        prop_assume!(window <= input.len());
        let has_nan = input.iter().all(|v| v.is_nan());
        let out = sma(&input, window);

        if has_nan {
            prop_assert!(out.is_err());
            prop_assert!(matches!(out, Err(TechnicalysisError::UnexpectedNan)));
        } else {
            let out = out.unwrap();
            prop_assert_eq!(out.len(), input.len());
            prop_assert!(out[..window-1].iter().all(|v| v.is_nan()));

            // Definition
            let slow = slow_sma(&input, window);
            for (o, expect) in out.iter().zip(slow) {
                if o.is_nan() || expect.is_nan() {
                    prop_assert!(o.is_nan() && expect.is_nan());
                } else {
                    prop_assert!(approx_eq_f64(*o, expect));
                }
            }

            // Linearity
            let k = 4.3;
            let scaled_input: Vec<_> = input.iter().map(|v| v * k).collect();
            let scaled_fast = sma(&scaled_input, window).unwrap();

            for (o, scaled) in out.iter().zip(scaled_fast) {
                if o.is_nan() {
                    prop_assert!(scaled.is_nan());
                } else {
                    prop_assert!(approx_eq_f64(scaled, o * k));
                }
            }
        }
    }
}
