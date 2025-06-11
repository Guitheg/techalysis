use crate::{
    expect_err_overflow_or_ok_with,
    helper::{
        assert::approx_eq_float,
        generated::{assert_vec_eq_gen_data, load_generated_csv},
    },
};

use techalysis::{
    errors::TechalysisError,
    indicators::trima::{trima, TrimaResult},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_trima(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let input_prev = &input[0..last_idx];

    let output = trima(input_prev, period);
    assert!(
        output.is_ok(),
        "Failed to calculate TRIMA: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        new_state.update(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(new_state.trima, expected[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected[last_idx + i],
            new_state.trima
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_trima("trima.csv", 30);
}

#[test]
fn generated_with_no_lookahead_period_25_ok() {
    generated_and_no_lookahead_trima("trima_timeperiod-25.csv", 25);
}

#[test]
fn generated_with_no_lookahead_period_2_ok() {
    generated_and_no_lookahead_trima("trima_timeperiod-2.csv", 2);
}

#[test]
fn generated_with_no_lookahead_period_3_ok() {
    generated_and_no_lookahead_trima("trima_timeperiod-3.csv", 3);
}

#[test]
fn zeros_ok() {
    let input = vec![0.0; 100];
    let result = trima(&input, 30).unwrap();
    assert!(result.values.iter().skip(29).all(|&v| v == 0.0));
}

#[test]
fn period_1_err() {
    let input = vec![0.0; 100];
    let result = trima(&input, 1);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::BadParam(_))));
}

#[test]
fn empty_input_err() {
    let data: [Float; 0] = [];
    let result = trima(&data, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn unexpected_nan_err() {
    let input = vec![1.0, 2.0, Float::NAN, 4.0];
    let result = trima(&input, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let input = vec![1.0, 2.0, Float::INFINITY, 4.0];
    let result = trima(&input, 3);
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
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(trima(&data, period), |result: TrimaResult| {
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
    let result = trima(&data, period).unwrap();
    let mut state = result.state;
    expect_err_overflow_or_ok_with!(state.update(Float::MIN + 5.0), |_| {
        assert!(state.trima.is_finite(), "Expected all values to be finite");
    });
}
