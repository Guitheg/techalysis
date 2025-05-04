use crate::oracle_test;
use crate::rust::tests_helper::assert::approx_eq_f64;
use crate::rust::tests_helper::{assert::assert_vec_close, oracle::read_fixture};
use proptest::{collection::vec, prelude::*};
use technicalysis::{errors::TechnicalysisError, features::ema::ema};

oracle_test!(ema, |x: &[f64]| ema(x, 30, 2.0));

#[test]
fn test_length_preserved() {
    let (input, _) = read_fixture("oracle/ema");
    let output = ema(&input, 30, 2.0);
    assert!(output.is_ok());
    let out = output.unwrap();
    assert_eq!(out.len(), input.len());
    assert!(out[..(30 - 1)].iter().all(|v| v.is_nan()));
}

#[test]
fn test_identity() {
    let (input, _) = read_fixture("oracle/ema");
    let output = ema(&input, 1, 2.0);
    assert!(output.is_ok());
    let out = output.unwrap();
    assert_vec_close(&out, &input);
}

#[test]
fn test_linearity() {
    let (input, expected) = read_fixture("oracle/ema");
    const K: f64 = 5.3;
    let scaled_input: Vec<f64> = input.iter().map(|v| v * K).collect();
    let scaled_expected: Vec<f64> = expected.iter().map(|v| v * K).collect();
    let output = ema(&scaled_input, 30, 2.0);
    assert!(output.is_ok());
    let out = output.unwrap();
    assert_vec_close(&out, &scaled_expected);
}

#[test]
fn test_extremum_value_injection_without_panic() {
    use std::f64;
    let data = vec![f64::MAX / 2.0, f64::MAX / 2.0, f64::MIN_POSITIVE, -0.0, 0.0];
    let out = ema(&data, 2, 2.0).expect("sma must not error on finite extremes");
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
    let result = ema(&data, 0, 2.0);
    assert!(result.is_err());
    if let Err(TechnicalysisError::BadParam(msg)) = result {
        assert!(msg.contains("between 2 and 100000"));
    }
}

#[test]
fn test_period_higher_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 3, 2.0);
    assert!(result.is_ok());
    let out = result.unwrap();
    assert!(out[2] == 2.0);
}

#[test]
fn test_unexpected_nan() {
    let data = vec![1.0, 2.0, 3.0, f64::NAN];
    let result = ema(&data, 3, 2.0);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::UnexpectedNan)));
}

#[test]
fn test_insufficient_data() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 4, 2.0);
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

proptest! {
    #[test]
    fn proptest_ema(
        input  in vec(-1e12f64..1e12, 2..200),
        window in 2usize..50
    ) {
        prop_assume!(window <= input.len());
        let has_nan = input.iter().all(|v| v.is_nan());
        let out = ema(&input, window, 2.0);

        if has_nan {
            prop_assert!(out.is_err());
            prop_assert!(matches!(out, Err(TechnicalysisError::UnexpectedNan)));
        } else {
            let out = out.unwrap();

            prop_assert_eq!(out.len(), input.len());
            prop_assert!(out[..window-1].iter().all(|v| v.is_nan()));

            let k = 7.0_f64;
            let scaled_input: Vec<_> = input.iter().map(|v| v*k).collect();
            let scaled_fast = ema(&scaled_input, window, 2.0).unwrap();

            for (orig, scaled) in out.iter().zip(&scaled_fast) {
                if orig.is_nan() {
                    prop_assert!(scaled.is_nan());
                } else {
                    let target = *orig * k;
                    prop_assert!(approx_eq_f64(target, *scaled),
                        "scaling fails: ema={}, k*ema={}, got={}", orig, target, scaled);
                }
            }
        }
    }
}
