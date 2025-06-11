use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalysis::{
    errors::TechalysisError,
    indicators::tema::{tema, tema_skip_period_unchecked, TemaResult},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_tema(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let input_prev = &input[0..last_idx];

    let output = tema(input_prev, period, None);
    assert!(
        output.is_ok(),
        "Failed to calculate TEMA: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        new_state.update(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(new_state.tema, expected[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected[last_idx + i],
            new_state.tema
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_tema("tema.csv", 30);
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
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(tema(&data, period, None), |result: TemaResult| {
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
    let result = tema(&data, period, None).unwrap();
    let mut state = result.state;
    expect_err_overflow_or_ok_with!(state.update(Float::MIN + 5.0), |_| {
        assert!(state.tema.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = tema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![
        1.0,
        2.0,
        Float::INFINITY,
        1.0,
        2.0,
        3.0,
        1.0,
        2.0,
        3.0,
        3.0,
        1.0,
        2.0,
    ];
    let period = 3;
    let result = tema(&data, period, None);
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
    let result = tema(&data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn large_period_exceeding_data_length() {
    let data = vec![1.0, 2.0, 3.0];
    let period = 10;
    let result = tema(&data, period, None);
    assert!(
        result.is_err(),
        "Expected an error for period exceeding data length, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn constant_values() {
    let data = vec![5.0; 10];
    let period = 3;
    let result = tema(&data, period, None);
    assert!(result.is_ok(), "Expected valid result for constant values");
    let tema_result = result.unwrap();
    assert!(
        tema_result
            .values
            .iter()
            .skip(tema_skip_period_unchecked(period))
            .all(|v| *v == 5.0),
        "Expected all values to be equal to the constant input"
    );
}
