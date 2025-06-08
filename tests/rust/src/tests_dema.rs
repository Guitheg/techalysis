use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalysis::{
    errors::TechalysisError,
    indicators::dema::{dema, dema_skip_period_unchecked, DemaResult, DemaState},
    types::Float,
};

#[test]
fn generated() {
    let columns = load_generated_csv("dema.csv").unwrap();

    let input = columns.get("close").unwrap();

    let out = columns.get("out").unwrap();

    let output = dema(input, 30, None);
    assert!(output.is_ok());
    let result = output.unwrap();

    assert_vec_eq_gen_data(out, &result.values);
}

#[test]
fn no_lookahead() {
    let columns = load_generated_csv("dema.csv").unwrap();

    let input = columns.get("close").unwrap();

    let len = input.len();
    let last_idx = len - 3;

    let out = columns.get("out").unwrap();

    let input_prev = &input[0..last_idx];

    let result = dema(input_prev, 30, None).unwrap();

    assert_vec_eq_gen_data(&out[0..last_idx], &result.values);

    let new_state = result.state.next(input[last_idx]).unwrap();
    assert!(
        approx_eq_float(new_state.dema, out[last_idx], 1e-8),
        "Expected last value to be {}, but got {}",
        out[last_idx],
        new_state.dema
    );
    let new_state = new_state.next(input[last_idx + 1]).unwrap();
    assert!(
        approx_eq_float(new_state.dema, out[last_idx + 1], 1e-8),
        "Expected last value to be {}, but got {}",
        out[last_idx],
        new_state.dema
    );
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
    expect_err_overflow_or_ok_with!(dema(&data, period, None), |result: DemaResult| {
        assert!(
            result.values.iter().skip(period + 1).all(|v| v.is_finite()),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;
    let result = dema(&data, period, None).unwrap();
    expect_err_overflow_or_ok_with!(result.state.next(Float::MIN + 5.0), |state: DemaState| {
        assert!(state.dema.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = dema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let period = 3;
    let result = dema(&data, period, None);
    assert!(
        result.is_err(),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn empty_input_err() {
    let data: [Float; 0] = [];
    let period = 14;
    let result = dema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn insufficient_data_err() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let period = 5;
    let result = dema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn constant_input() {
    let data = vec![10.0; 50];
    let period = 5;
    let result = dema(&data, period, None).unwrap();
    assert!(
        result
            .values
            .iter()
            .skip(dema_skip_period_unchecked(period))
            .all(|&v| approx_eq_float(v, 10.0, 1e-8)),
        "Expected all values to be approximately 10.0"
    );
}

#[test]
fn increasing_input() {
    let data: Vec<Float> = (1..=50).map(|x| x as Float).collect();
    let period = 5;
    let result = dema(&data, period, None).unwrap();
    assert!(
        result
            .values
            .iter()
            .zip(data.iter())
            .skip(dema_skip_period_unchecked(period))
            .all(|(out, &inp)| approx_eq_float(*out, inp, 1e-8)),
        "Expected DEMA values to be less than or equal to the input values"
    );
}

#[test]
fn decreasing_input() {
    let data: Vec<Float> = (1..=50).rev().map(|x| x as Float).collect();
    let period = 5;
    let result = dema(&data, period, None).unwrap();
    assert!(
        result
            .values
            .iter()
            .zip(data.iter())
            .skip(dema_skip_period_unchecked(period))
            .all(|(out, &inp)| approx_eq_float(*out, inp, 1e-8)),
        "Expected DEMA values to be greater than or equal to the input values"
    );
}
