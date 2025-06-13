use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::midprice::{midprice, MidpriceResult, MidpriceSample},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_midprice(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let high_prices = columns.get("high").unwrap();
    let low_prices = columns.get("low").unwrap();

    let len = high_prices.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let high_prices_prev = &high_prices[0..last_idx];
    let low_prices_prev = &low_prices[0..last_idx];

    let output = midprice(high_prices_prev, low_prices_prev, period);
    assert!(
        output.is_ok(),
        "Failed to calculate MIDPRICE: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        new_state
            .update(&MidpriceSample {
                high: high_prices[last_idx + i],
                low: low_prices[last_idx + i],
            })
            .unwrap();
        assert!(
            approx_eq_float(new_state.midprice, expected[last_idx + i], 1e-8),
            "Next [{}] expected {}, but got {}",
            i,
            expected[last_idx + i],
            new_state.midprice
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_midprice("midprice.csv", 14)
}

#[test]
fn finite_extreme_err_overflow_or_ok_all_finite() {
    let high_prices = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let low_prices = vec![
        Float::MAX - 5.0,
        Float::MAX - 9.0,
        Float::MAX - 9.0,
        Float::MAX - 8.0,
        Float::MAX - 10.0,
        Float::MAX - 11.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(
        midprice(&high_prices, &low_prices, period),
        |result: MidpriceResult| {
            assert!(
                result.values.iter().skip(period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
        }
    );
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let high_prices = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let low_prices = vec![3.0, 8.0, 25.0, 1.0, 3.5, 3.5, 4.0];
    let period = 3;
    let result = midprice(&high_prices, &low_prices, period).unwrap();
    let mut state = result.state;
    let midprice_sample = MidpriceSample {
        high: Float::MIN + 50.0,
        low: Float::MIN + 1.0,
    };
    expect_err_overflow_or_ok_with!(state.update(&midprice_sample), |_| {
        assert!(
            state.midprice.is_finite(),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn unexpected_nan_err() {
    let high_prices = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let low_prices = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = midprice(&high_prices, &low_prices, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let high_prices = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let low_prices = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let period = 3;
    let result = midprice(&high_prices, &low_prices, period);
    assert!(
        result.is_err(),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn empty_input_err() {
    let high_prices: [Float; 0] = [];
    let low_prices: [Float; 0] = [];
    let period = 14;
    let result = midprice(&high_prices, &low_prices, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn different_length_err() {
    let high_prices = vec![1.0, 2.0, 3.0];
    let low_prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let period = 2;
    let result = midprice(&high_prices, &low_prices, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}
