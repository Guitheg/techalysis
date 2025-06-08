use crate::{
    expect_err_overflow_or_ok_with,
    helper::{
        assert::approx_eq_float,
        generated::{assert_vec_eq_gen_data, load_generated_csv},
    },
};

use techalysis::{
    errors::TechalysisError,
    indicators::bbands::{bbands, BBandsMA, BBandsResult, BBandsState, DeviationMulipliers},
    types::Float,
};

#[test]
fn generated() {
    let columns = load_generated_csv("bbands.csv").unwrap();

    let input = columns.get("close").unwrap();

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let output = bbands(
        input,
        20,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    );
    assert!(output.is_ok());
    let result = output.unwrap();

    assert_vec_eq_gen_data(upper, &result.upper);
    assert_vec_eq_gen_data(middle, &result.middle);
    assert_vec_eq_gen_data(lower, &result.lower);
}

#[test]
fn generated_ema() {
    let columns = load_generated_csv("bbands_matype-1.csv").unwrap();

    let input = columns.get("close").unwrap();

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let output = bbands(
        input,
        20,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::EMA(None),
    );
    assert!(output.is_ok());
    let result = output.unwrap();

    assert_vec_eq_gen_data(upper, &result.upper);
    assert_vec_eq_gen_data(middle, &result.middle);
    assert_vec_eq_gen_data(lower, &result.lower);
}

#[test]
fn no_lookahead() {
    let columns = load_generated_csv("bbands.csv").unwrap();

    let input = columns.get("close").unwrap();

    let len = input.len();
    let last_idx = len - 2;

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let input_prev = &input[0..last_idx];

    let result = bbands(
        input_prev,
        20,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    )
    .unwrap();

    assert_vec_eq_gen_data(&upper[0..last_idx], &result.upper);
    assert_vec_eq_gen_data(&middle[0..last_idx], &result.middle);
    assert_vec_eq_gen_data(&lower[0..last_idx], &result.lower);

    let new_state = result.state.next(input[last_idx]).unwrap();
    assert!(
        approx_eq_float(new_state.upper, upper[last_idx], 1e-8),
        "Expected last value to be {}, but got {}",
        upper[last_idx],
        new_state.upper
    );
}

#[test]
fn no_lookahead_ema() {
    let columns = load_generated_csv("bbands_matype-1.csv").unwrap();

    let input = columns.get("close").unwrap();

    let len = input.len();
    let last_idx = len - 3;

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let input_prev = &input[0..last_idx];

    let result = bbands(
        input_prev,
        20,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::EMA(None),
    )
    .unwrap();

    assert_vec_eq_gen_data(&upper[0..last_idx], &result.upper);
    assert_vec_eq_gen_data(&middle[0..last_idx], &result.middle);
    assert_vec_eq_gen_data(&lower[0..last_idx], &result.lower);

    let new_state = result.state.next(input[last_idx]).unwrap();
    assert!(
        approx_eq_float(new_state.upper, upper[last_idx], 1e-8),
        "Expected last value to be {}, but got {}",
        upper[last_idx],
        new_state.upper
    );
    let new_state = new_state.next(input[last_idx + 1]).unwrap();
    assert!(
        approx_eq_float(new_state.upper, upper[last_idx + 1], 1e-8),
        "Expected last value to be {}, but got {}",
        upper[last_idx],
        new_state.upper
    );
}

#[test]
fn all_zeros() {
    let n = 30;
    let input = vec![0.0; n];
    let result = bbands(
        &input,
        10,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    )
    .unwrap();
    let expected = vec![Float::NAN; 9]
        .into_iter()
        .chain(vec![0.0; n - 9])
        .collect::<Vec<Float>>();
    assert_vec_eq_gen_data(&expected, &result.upper);
    assert_vec_eq_gen_data(&expected, &result.middle);
    assert_vec_eq_gen_data(&expected, &result.lower);
}

#[test]
fn linear_series_stability() {
    let input: Vec<Float> = (0..50).map(|x| x as Float).collect();
    let result = bbands(
        &input,
        5,
        DeviationMulipliers { up: 1.5, down: 1.5 },
        BBandsMA::SMA,
    )
    .unwrap();
    for i in 5..result.middle.len() {
        let diff = result.middle[i] - result.middle[i - 1];
        assert!((diff - 1.0).abs() < 1e-6);
    }
}

#[test]
fn breakout_detection() {
    let mut input: Vec<Float> = vec![100.0; 20];
    for (i, item) in input.iter_mut().enumerate().take(20) {
        *item += i as Float;
    }
    input.push(200.0); // sudden spike
    let len = input.len();
    let result = bbands(
        &input,
        20,
        DeviationMulipliers { up: 1.0, down: 1.0 },
        BBandsMA::SMA,
    )
    .unwrap();
    let last_price = input[len - 1];
    let upper = result.upper[len - 1];
    assert!(last_price > upper, "Expected breakout above upper band");
}

#[test]
fn nan_input_err() {
    let mut input = vec![1.0, 2.0, 3.0];
    input.push(Float::NAN);
    let output = bbands(
        &input,
        3,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    );
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn invalid_params_err() {
    let data = vec![1.0, 2.0, 3.0];
    let output = bbands(
        &data,
        0,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    );
    assert!(output.is_err()); // length = 0
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));

    let output = bbands(
        &data,
        3,
        DeviationMulipliers {
            up: -1.0,
            down: 2.0,
        },
        BBandsMA::SMA,
    );
    assert!(output.is_err()); // negative std_dev mult
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));

    let output = bbands(
        &data,
        3,
        DeviationMulipliers {
            up: 2.0,
            down: -1.0,
        },
        BBandsMA::SMA,
    );
    assert!(output.is_err()); // negative lower mult
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));
}

#[test]
fn insufficient_data_err() {
    let input = vec![1.0, 2.0, 3.0];
    let output = bbands(
        &input,
        5,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    );
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::InsufficientData)),);
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 5.0, 6.0, 7.0];
    let result = bbands(
        &data,
        3,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    );
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, 3.0, Float::INFINITY, 5.0, 6.0, 7.0];
    let result = bbands(
        &data,
        3,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    );
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn next_with_finite_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;

    let result = bbands(
        &data,
        period,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        BBandsMA::SMA,
    )
    .unwrap();
    expect_err_overflow_or_ok_with!(result.state.next(Float::MAX - 5.0), |state: BBandsState| {
        assert!(state.upper.is_finite(), "Expected all values to be finite");
        assert!(state.middle.is_finite(), "Expected all values to be finite");
        assert!(state.lower.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![
        Float::MIN + 3.0,
        Float::MIN + 2.0,
        Float::MIN + 5.0,
        Float::MIN + 6.0,
        Float::MIN + 8.0,
        Float::MIN + 1.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(
        bbands(
            &data,
            period,
            DeviationMulipliers { up: 2.0, down: 2.0 },
            BBandsMA::SMA
        ),
        |result: BBandsResult| {
            assert!(
                result.upper.iter().skip(period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
            assert!(
                result.middle.iter().skip(period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
            assert!(
                result.lower.iter().skip(period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
        }
    );
}
