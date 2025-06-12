use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::dema::{dema, dema_skip_period_unchecked, DemaResult},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_dema(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let input_prev = &input[0..last_idx];

    let output = dema(input_prev, period, None);
    assert!(
        output.is_ok(),
        "Failed to calculate DEMA: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        new_state.update(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(new_state.dema, expected[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected[last_idx + i],
            new_state.dema
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_dema("dema.csv", 30);
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
    let mut state = result.state;
    expect_err_overflow_or_ok_with!(state.update(Float::MIN + 5.0), |_| {
        assert!(
            state.dema.is_finite(),
            "Expected all values to be finite after next call"
        );
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = dema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
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
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn empty_input_err() {
    let data: [Float; 0] = [];
    let period = 14;
    let result = dema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn insufficient_data_err() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let period = 5;
    let result = dema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
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
