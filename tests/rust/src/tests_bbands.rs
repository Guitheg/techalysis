use crate::{
    expect_err_overflow_or_ok_with,
    helper::{
        assert::{approx_eq_float},
        generated::{assert_vec_eq_gen_data, load_generated_csv},
    },
};

use techalysis::{
    errors::TechalysisError,
    indicators::bbands::{bbands, BBandsMA, BBandsResult, DeviationMulipliers},
    types::Float,
};

fn generated_and_no_lookahead_bbands(file_name: &str, period: usize, ma_type: BBandsMA) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let output = bbands(
        &input[0..last_idx],
        period,
        DeviationMulipliers { up: 2.0, down: 2.0 },
        ma_type,
    );
    assert!(output.is_ok(), "Failed to calculate BBands: {:?}", output.err());
    let result = output.unwrap();

    assert_vec_eq_gen_data(&upper[0..last_idx], &result.upper);
    assert_vec_eq_gen_data(&middle[0..last_idx], &result.middle);
    assert_vec_eq_gen_data(&lower[0..last_idx], &result.lower);

    let mut state = result.state;

    for i in 0..next_count {
        state.next(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(state.upper, upper[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            upper[last_idx + i],
            state.upper
        );
        assert!(
            approx_eq_float(state.middle, middle[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            middle[last_idx + i],
            state.middle
        );
        assert!(
            approx_eq_float(state.lower, lower[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            lower[last_idx + i],
            state.lower
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_bbands(
        "bbands.csv",
        20,
        BBandsMA::SMA,
    );
}

#[test]
fn generated_with_no_lookahead_ema_ok() {
    generated_and_no_lookahead_bbands(
        "bbands_matype-1.csv",
        20,
        BBandsMA::EMA(None),
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
    let mut state = result.state;
    let output = state.next(Float::MAX - 5.0);
    expect_err_overflow_or_ok_with!(output, |_| {
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
