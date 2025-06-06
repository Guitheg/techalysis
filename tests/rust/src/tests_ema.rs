use crate::helper::{
    assert::approx_eq_f64, generated::assert_vec_eq_gen_data, generated::load_generated_csv,
};
use proptest::{collection::vec, prelude::*};
use technicalysis::{errors::TechnicalysisError, indicators::ema::ema};

#[test]
fn generated() {
    let columns = load_generated_csv("ema.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = ema(input, 30, None);
    assert!(output.is_ok());
    let out = output.unwrap().values;
    let expected = columns.get("out").unwrap();
    assert_vec_eq_gen_data(&out, expected);
    assert!(out.len() == input.len());
}

#[test]
fn no_lookahead() {
    let columns = load_generated_csv("ema.csv").unwrap();
    let input = columns.get("close").unwrap();
    let expected = columns.get("out").unwrap();
    let output = ema(&input[0..input.len() - 1], 30, None);
    assert!(output.is_ok());
    let result = output.unwrap();
    assert_vec_eq_gen_data(&result.values, &expected[0..&expected.len() - 1]);
    let new_output = result.state.next(input[input.len() - 1]);
    assert!(new_output.is_ok());
    let new_state = new_output.unwrap();
    assert!(
        approx_eq_f64(new_state.ema, expected[expected.len() - 1]),
        "Expected last value to be {}, but got {}",
        expected[expected.len() - 1],
        new_state.ema
    );
}

#[test]
fn extremum_value_injection_without_panic() {
    use std::f64;
    let data = vec![f64::MAX / 2.0, f64::MAX / 2.0, f64::MIN_POSITIVE, -0.0, 0.0];
    let out = ema(&data, 2, None)
        .expect("sma must not error on finite extremes")
        .values;
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
fn invalid_period_lower_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 0, None);
    assert!(result.is_err());
    if let Err(TechnicalysisError::BadParam(msg)) = result {
        assert!(msg.contains("between 2 and 100000"));
    }
}

#[test]
fn period_higher_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 3, None);
    assert!(result.is_ok());
    let out = result.unwrap().values;
    assert!(out[2] == 2.0);
}

#[test]
fn unexpected_nan() {
    let data = vec![1.0, 2.0, 3.0, f64::NAN];
    let result = ema(&data, 3, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::UnexpectedNan)));
}

#[test]
fn insufficient_data() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 4, None);
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}

#[test]
fn period_1() {
    let data = &[10.0, 11.0, 10.0, 10.0, 12.0];
    let period = 1;
    let result = ema(data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechnicalysisError::BadParam(_))));
}

proptest! {
    #[test]
    fn proptest(
        input  in vec(-1e12f64..1e12, 2..200),
        window in 2usize..50
    ) {
        prop_assume!(window <= input.len());
        let has_nan = input.iter().all(|v| v.is_nan());
        let out = ema(&input, window, None);

        if has_nan {
            prop_assert!(out.is_err());
            prop_assert!(matches!(out, Err(TechnicalysisError::UnexpectedNan)));
        } else {
            let out = out.unwrap().values;

            prop_assert_eq!(out.len(), input.len());
            prop_assert!(out[..window-1].iter().all(|v| v.is_nan()));

            let k = 7.0_f64;
            let scaled_input: Vec<_> = input.iter().map(|v| v*k).collect();
            let scaled_fast = ema(&scaled_input, window, None).unwrap().values;

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
