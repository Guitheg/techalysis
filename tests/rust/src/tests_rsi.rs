use crate::helper::{
    assert::{approx_eq_float, assert_vec_close},
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};
use crate::{assert_vec_float_eq, expect_err_overflow_or_ok_with};
use proptest::{prop_assert, prop_assert_eq, proptest};
use techalysis::{
    errors::TechalysisError,
    indicators::rsi::{rsi, RsiResult, RsiState},
    types::Float,
};

#[test]
fn generated() {
    let columns = load_generated_csv("rsi.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = rsi(input, 14);
    assert!(output.is_ok());
    let out = output.unwrap().values;
    let expected = columns.get("out").unwrap();
    assert_vec_eq_gen_data(&out, expected);
    assert!(out.len() == input.len());
}

#[test]
fn no_lookahead() {
    let columns = load_generated_csv("rsi.csv").unwrap();
    let input = columns.get("close").unwrap();
    let len = input.len();
    let input_minus_1 = &input[0..len - 1];
    let last_input = input[len - 1];
    let expected = columns.get("out").unwrap();
    let expected_minus_1 = &expected[0..len - 1];
    let last_expected = expected[len - 1];

    let output_minus_1 = rsi(input_minus_1, 14);
    assert!(output_minus_1.is_ok());
    let result_minus_1 = output_minus_1.unwrap();
    assert_vec_eq_gen_data(&result_minus_1.values, expected_minus_1);
    let output = rsi(input, 14).unwrap();
    assert_vec_eq_gen_data(&output.values[0..len - 1], &result_minus_1.values);
    let next_state = result_minus_1.state.next(last_input).unwrap();
    assert!(
        approx_eq_float(next_state.rsi, last_expected, 1e-8),
        "Expected last value to be {}, but got {}",
        expected[len - 1],
        next_state.rsi
    );
    assert!(
        approx_eq_float(next_state.rsi, output.state.rsi, 1e-8),
        "Expected last value to be {}, but got {}",
        output.state.rsi,
        next_state.rsi
    );
}

#[test]
fn empty_input() {
    let data: [Float; 0] = [];
    let period = 14;
    let result = rsi(&data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn input_shorter_than_period() {
    let data = &[1.0, 2.0, 3.0];
    let period = 5;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn input_length_equals_period() {
    let data = &[1.0, 2.0, 3.0];
    let period = 3;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn period_1() {
    let data = &[10.0, 11.0, 10.0, 10.0, 12.0];
    let period = 1;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::BadParam(_))));
}

#[test]
fn min_data_for_one_value_mixed() {
    let data = &[10.0, 11.0, 10.0];
    let period = 2;
    let expected = vec![Float::NAN, Float::NAN, 50.0];
    let result = rsi(data, period).unwrap().values;
    assert_vec_close(&result, &expected);
}

#[test]
fn alternating_up_down() {
    let data = &[10.0, 12.0, 10.0, 12.0, 10.0, 12.0];
    let period = 2;
    let expected = vec![Float::NAN, Float::NAN, 50.0, 75.0, 37.5, 68.75];
    let result = rsi(data, period).unwrap().values;
    assert_vec_close(&result, &expected);
}

#[test]
fn all_values_approximatelly_same() {
    let data = &[
        10.1, 9.9, 10.0, 10.2, 10.1, 10.12, 10.11, 10.12, 10.11, 10.10, 10.09, 10.10,
    ];
    let period = 3;
    let expected = vec![
        Float::NAN,
        Float::NAN,
        Float::NAN,
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
    let result = rsi(data, period).unwrap().values;
    assert_vec_float_eq!(&result, &expected, 1e-6);
}

#[test]
fn all_values_same() {
    let data = &[10.0, 10.0, 10.0, 10.0, 10.0];
    let period = 3;
    let expected = vec![Float::NAN, Float::NAN, Float::NAN, 50.0, 50.0];
    let result = rsi(data, period).unwrap().values;
    assert_vec_close(&result, &expected);
}

#[test]
fn all_increasing() {
    let data = &[1.0, 2.0, 3.0, 4.0, 5.0];
    let period = 3;
    let expected = vec![Float::NAN, Float::NAN, Float::NAN, 100.0, 100.0];
    let result = rsi(data, period).unwrap().values;
    assert_vec_close(&result, &expected);
}

#[test]
fn all_decreasing() {
    let data = &[5.0, 4.0, 3.0, 2.0, 1.0];
    let period = 3;
    let expected = vec![Float::NAN, Float::NAN, Float::NAN, 0.0, 0.0];
    let result = rsi(data, period).unwrap().values;
    assert_vec_close(&result, &expected);
}

#[test]
fn input_with_nans() {
    let data = &[1.0, 2.0, Float::NAN, 4.0, 5.0];
    let period = 2;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn period_zero() {
    let data = &[1.0, 2.0, 3.0];
    let period = 0;
    let result = rsi(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 5.0, 6.0, 7.0];
    let result = rsi(&data, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, 3.0, Float::INFINITY, 5.0, 6.0, 7.0];
    let result = rsi(&data, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn finite_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(rsi(&data, period), |result: RsiResult| {
        assert!(
            result.values.iter().skip(period).all(|v| v.is_finite()),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;
    let result = rsi(&data, period).unwrap();
    expect_err_overflow_or_ok_with!(result.state.next(Float::MIN + 5.0), |result: RsiState| {
        assert!(result.rsi.is_finite(), "Expected all values to be finite");
    });
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
                prop_assert!(matches!(result, Err(TechalysisError::BadParam(_))));
            } else {
                prop_assert!(matches!(result, Err(TechalysisError::InsufficientData)));
            }
        } else {
            let rsi_values = result.unwrap().values;
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
