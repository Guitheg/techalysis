use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use techalysis::{
    errors::TechalysisError, indicators::t3::{
        t3,
        T3Result,
    }, traits::State, types::Float
};
use crate::expect_err_overflow_or_ok_with;

fn generated_and_no_lookahead_t3(file_name: &str, period: usize, vfactor: Float) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let input_prev = &input[0..last_idx];

    let output = t3(input_prev, period,vfactor, None);
    assert!(output.is_ok(), "Failed to calculate T3: {:?}", output.err());
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        new_state.update(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(new_state.t3, expected[last_idx + i], 1e-8),
            "Next({}) expected {}, but got {}",
            i,
            expected[last_idx + i],
            new_state.t3
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_t3(
        "t3.csv",
        20,
        0.7,
    )
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
        Float::MAX - 4.0,
        Float::MAX - 2.0,
        Float::MAX - 3.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
        Float::MAX - 4.0,
        Float::MAX - 2.0,
        Float::MAX - 3.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(
        t3(&data, period, 0.7, None),
        |result: T3Result| {
            assert!(
                result.values.iter().skip(period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
        }
    );
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0, 1.0, 2.0, 3.0, 
        4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 30.0, 20.0, 30.0, 3.0];
    let period = 3;
    let result = t3(&data, period, 0.7, None).unwrap();
    let mut state = result.state;
    expect_err_overflow_or_ok_with!(state.update(Float::MIN + 5.0), |_| {
        assert!(state.t3.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0, 5.0, 3.0, 2.0,
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let period = 3;
    let result = t3(&data, period, 0.7, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0,
    2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0];
    let period = 3;
    let result =  t3(&data, period, 0.7, None);
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
    let result = t3(&data, period, 0.7, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}
