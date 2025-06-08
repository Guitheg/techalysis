use techalysis::{
    errors::TechalysisError,
    indicators::macd::{macd, MacdResult, MacdState},
    types::Float,
};

use crate::{
    expect_err_overflow_or_ok_with,
    helper::{
        assert::approx_eq_float,
        generated::{assert_vec_eq_gen_data, load_generated_csv},
    },
};

#[test]
fn generated_default() {
    let columns = load_generated_csv("macd.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = macd(input, 12, 26, 9);
    assert!(output.is_ok());
    let out = output.unwrap();
    let expected_macd = columns.get("macd").unwrap();
    let expected_signal = columns.get("signal").unwrap();
    let expected_histogram = columns.get("histogram").unwrap();
    assert_vec_eq_gen_data(&out.macd, expected_macd);
    assert_vec_eq_gen_data(&out.signal, expected_signal);
    assert_vec_eq_gen_data(&out.histogram, expected_histogram);
    assert!(out.macd.len() == input.len());
}

#[test]
fn generated_fast16_slow36_signal12() {
    let columns =
        load_generated_csv("macd_fastperiod-16_slowperiod-36_signalperiod-12.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = macd(input, 16, 36, 12);
    assert!(output.is_ok());
    let out = output.unwrap();
    let expected_macd = columns.get("macd").unwrap();
    let expected_signal = columns.get("signal").unwrap();
    let expected_histogram = columns.get("histogram").unwrap();
    assert_vec_eq_gen_data(&out.macd, expected_macd);
    assert_vec_eq_gen_data(&out.signal, expected_signal);
    assert_vec_eq_gen_data(&out.histogram, expected_histogram);
    assert!(out.macd.len() == input.len());
}

#[test]
fn no_lookahead() {
    let columns = load_generated_csv("macd.csv").unwrap();
    let input = columns.get("close").unwrap();
    let len = input.len();
    let last_index = len - 1;
    let input_minus = &input[0..last_index];
    let expected_macd = columns.get("macd").unwrap();
    let expected_macd_minus = &expected_macd[0..last_index];
    let expected_signal = columns.get("signal").unwrap();
    let expected_signal_minus = &expected_signal[0..last_index];
    let expected_histogram = columns.get("histogram").unwrap();
    let expected_histogram_minus = &expected_histogram[0..last_index];

    let result_minus = macd(input_minus, 12, 26, 9).unwrap();
    assert_vec_eq_gen_data(&result_minus.macd, expected_macd_minus);
    assert_vec_eq_gen_data(&result_minus.signal, expected_signal_minus);
    assert_vec_eq_gen_data(&result_minus.histogram, expected_histogram_minus);

    let next_state = result_minus.state.next(input[last_index]).unwrap();
    assert!(
        approx_eq_float(next_state.macd, expected_macd[last_index], 1e-8),
        "Expected last MACD value to be {}, but got {}",
        expected_macd[last_index],
        next_state.macd
    );
    assert!(
        approx_eq_float(next_state.signal, expected_signal[last_index], 1e-8),
        "Expected last Signal value to be {}, but got {}",
        expected_signal[last_index],
        next_state.signal
    );
    assert!(
        approx_eq_float(next_state.histogram, expected_histogram[last_index], 1e-8),
        "Expected last Histogram value to be {}, but got {}",
        expected_histogram[last_index],
        next_state.histogram
    );

    let result = macd(input, 12, 26, 9).unwrap();
    assert_vec_eq_gen_data(&result.macd, expected_macd);
    assert_vec_eq_gen_data(&result.signal, expected_signal);
    assert_vec_eq_gen_data(&result.histogram, expected_histogram);
    assert!(result.state.macd == next_state.macd);
    assert!(result.state.signal == next_state.signal);
    assert!(result.state.histogram == next_state.histogram);
}

#[test]
fn generated_signal32() {
    let columns = load_generated_csv("macd_signalperiod-32.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = macd(input, 12, 26, 32);
    assert!(output.is_ok());
    let out = output.unwrap();
    let expected_macd = columns.get("macd").unwrap();
    let expected_signal = columns.get("signal").unwrap();
    let expected_histogram = columns.get("histogram").unwrap();
    assert_vec_eq_gen_data(&out.macd, expected_macd);
    assert_vec_eq_gen_data(&out.signal, expected_signal);
    assert_vec_eq_gen_data(&out.histogram, expected_histogram);
    assert!(out.macd.len() == input.len());
}

#[test]
fn empty_input() {
    let input: Vec<Float> = vec![];
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(
        matches!(output, Err(TechalysisError::InsufficientData)),
        "Got: {output:?}"
    );
}

#[test]
fn uniform_data() {
    let input: Vec<Float> = vec![100.0; 50];
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_ok());
    let out = output.unwrap();
    let macd_nonzero = out.macd.iter().filter(|&&x| x.abs() > 1e-6).count();
    let signal_nonzero = out.signal.iter().filter(|&&x| x.abs() > 1e-6).count();
    let histogram_nonzero = out.histogram.iter().filter(|&&x| x.abs() > 1e-6).count();
    assert_eq!(macd_nonzero, 0);
    assert_eq!(signal_nonzero, 0);
    assert_eq!(histogram_nonzero, 0);
}

#[test]
fn insufficient_data() {
    let input: Vec<Float> = (1..=20).map(|x| x as Float).collect();
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(
        matches!(output, Err(TechalysisError::InsufficientData)),
        "Got: {output:?}",
    );
}

#[test]
fn fast_greater_than_slow() {
    let input: Vec<Float> = (1..=50).map(|x| x as Float).collect();
    let output = macd(&input, 30, 20, 9);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::BadParam(_))));
}

#[test]
fn unexpected_nan_err() {
    let mut input: Vec<Float> = (1..=50).map(|x| x as Float).collect();
    input[10] = Float::NAN;
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let mut input: Vec<Float> = (1..=50).map(|x| x as Float).collect();
    input[10] = Float::INFINITY;
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechalysisError::DataNonFinite(_))));
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
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let fast_period = 3;
    let slow_period = 4;
    let signal_period = 2;
    let skip_period = slow_period + signal_period - 2;

    expect_err_overflow_or_ok_with!(
        macd(&data, fast_period, slow_period, signal_period),
        |result: MacdResult| {
            assert!(
                result.macd.iter().skip(skip_period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
            assert!(
                result
                    .signal
                    .iter()
                    .skip(skip_period)
                    .all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
            assert!(
                result
                    .histogram
                    .iter()
                    .skip(skip_period)
                    .all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
        }
    );
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0, 30.0, 3.0, 5.0, 6.0];
    let fast_period = 3;
    let slow_period = 4;
    let signal_period = 2;

    let result = macd(&data, fast_period, slow_period, signal_period).unwrap();
    expect_err_overflow_or_ok_with!(result.state.next(Float::MIN + 5.0), |state: MacdState| {
        assert!(state.macd.is_finite(), "Expected all values to be finite");
        assert!(state.signal.is_finite(), "Expected all values to be finite");
        assert!(
            state.histogram.is_finite(),
            "Expected all values to be finite"
        );
    });
}
