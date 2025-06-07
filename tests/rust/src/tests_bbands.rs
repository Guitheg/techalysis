use crate::helper::{
    assert::approx_eq_f64_custom,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use techalysis::{
    errors::TechalysisError,
    indicators::bbands::{bbands, BBandsMA},
};

#[test]
fn generated() {
    let columns = load_generated_csv("bbands.csv").unwrap();

    let input = columns.get("close").unwrap();

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let output = bbands(input, 20, 2.0, 2.0, BBandsMA::SMA);
    assert!(output.is_ok());
    let result = output.unwrap();

    assert_vec_eq_gen_data(upper, &result.upper_band);
    assert_vec_eq_gen_data(middle, &result.middle_band);
    assert_vec_eq_gen_data(lower, &result.lower_band);
}

#[test]
fn generated_ema() {
    let columns = load_generated_csv("bbands_matype-1.csv").unwrap();

    let input = columns.get("close").unwrap();

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let output = bbands(input, 20, 2.0, 2.0, BBandsMA::EMA(None));
    assert!(output.is_ok());
    let result = output.unwrap();

    assert_vec_eq_gen_data(upper, &result.upper_band);
    assert_vec_eq_gen_data(middle, &result.middle_band);
    assert_vec_eq_gen_data(lower, &result.lower_band);
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

    let result = bbands(input_prev, 20, 2.0, 2.0, BBandsMA::SMA).unwrap();

    assert_vec_eq_gen_data(&upper[0..last_idx], &result.upper_band);
    assert_vec_eq_gen_data(&middle[0..last_idx], &result.middle_band);
    assert_vec_eq_gen_data(&lower[0..last_idx], &result.lower_band);

    let new_state = result.state.next(input[last_idx]).unwrap();
    assert!(
        approx_eq_f64_custom(new_state.upper, upper[last_idx], 1e-8),
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

    let result = bbands(input_prev, 20, 2.0, 2.0, BBandsMA::EMA(None)).unwrap();

    assert_vec_eq_gen_data(&upper[0..last_idx], &result.upper_band);
    assert_vec_eq_gen_data(&middle[0..last_idx], &result.middle_band);
    assert_vec_eq_gen_data(&lower[0..last_idx], &result.lower_band);

    let new_state = result.state.next(input[last_idx]).unwrap();
    assert!(
        approx_eq_f64_custom(new_state.upper, upper[last_idx], 1e-8),
        "Expected last value to be {}, but got {}",
        upper[last_idx],
        new_state.upper
    );
    let new_state = new_state.next(input[last_idx + 1]).unwrap();
    assert!(
        approx_eq_f64_custom(new_state.upper, upper[last_idx + 1], 1e-8),
        "Expected last value to be {}, but got {}",
        upper[last_idx],
        new_state.upper
    );
}

#[test]
fn all_zeros() {
    let n = 30;
    let input = vec![0.0; n];
    let result = bbands(&input, 10, 2.0, 2.0, BBandsMA::SMA).unwrap();
    let expected = vec![f64::NAN; 9]
        .into_iter()
        .chain(vec![0.0; n - 9])
        .collect::<Vec<f64>>();
    assert_vec_eq_gen_data(&expected, &result.upper_band);
    assert_vec_eq_gen_data(&expected, &result.middle_band);
    assert_vec_eq_gen_data(&expected, &result.lower_band);
}

#[test]
fn linear_series_stability() {
    let input: Vec<f64> = (0..50).map(|x| x as f64).collect();
    let result = bbands(&input, 5, 1.5, 1.5, BBandsMA::SMA).unwrap();
    for i in 5..result.middle_band.len() {
        let diff = result.middle_band[i] - result.middle_band[i - 1];
        assert!((diff - 1.0).abs() < 1e-6);
    }
}

#[test]
fn breakout_detection() {
    let mut input: Vec<f64> = vec![100.0; 20];
    for i in 0..20 {
        input[i] += i as f64;
    }
    input.push(200.0); // sudden spike
    let len = input.len();
    let result = bbands(&input, 20, 1.0, 1.0, BBandsMA::SMA).unwrap();
    let last_price = input[len - 1];
    let upper = result.upper_band[len - 1];
    assert!(last_price > upper, "Expected breakout above upper band");
}

#[test]
fn nan_input_err() {
    let mut input = vec![1.0, 2.0, 3.0];
    input.push(f64::NAN);
    let output = bbands(&input, 3, 2.0, 2.0, BBandsMA::SMA);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::UnexpectedNan)));
}

#[test]
fn invalid_params_err() {
    let data = vec![1.0, 2.0, 3.0];
    let output = bbands(&data, 0, 2.0, 2.0, BBandsMA::SMA);
    assert!(output.is_err()); // length = 0
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));

    let output = bbands(&data, 3, -1.0, 2.0, BBandsMA::SMA);
    assert!(output.is_err()); // negative std_dev mult
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));

    let output = bbands(&data, 3, 2.0, -1.0, BBandsMA::SMA);
    assert!(output.is_err()); // negative lower mult
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));
}

#[test]
fn insufficient_data_err() {
    let input = vec![1.0, 2.0, 3.0];
    let output = bbands(&input, 5, 2.0, 2.0, BBandsMA::SMA);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::InsufficientData)),);
}
