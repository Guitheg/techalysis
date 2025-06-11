use techalysis::{
    errors::TechalysisError, indicators::macd::{macd, MacdResult}, traits::State, types::Float
};

use crate::{
    expect_err_overflow_or_ok_with,
    helper::{
        assert::approx_eq_float,
        generated::{assert_vec_eq_gen_data, load_generated_csv},
    },
};

fn generated_and_no_lookahead_macd(file_name: &str, fast_period: usize, slow_period: usize, signal_period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected_macd = columns.get("macd").unwrap();
    let expected_signal = columns.get("signal").unwrap();
    let expected_histogram = columns.get("histogram").unwrap();

    let output = macd(
        &input[0..last_idx],
        fast_period,
        slow_period,
        signal_period,
    );
    assert!(output.is_ok(), "Failed to calculate MACD: {:?}", output.err());
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected_macd[0..last_idx], &result.macd);
    assert_vec_eq_gen_data(&expected_signal[0..last_idx], &result.signal);
    assert_vec_eq_gen_data(&expected_histogram[0..last_idx], &result.histogram);

    let mut state = result.state;

    for i in 0..next_count {
        state.update(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(state.macd, expected_macd[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected_macd[last_idx + i],
            state.macd
        );
        assert!(
            approx_eq_float(state.signal, expected_signal[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected_signal[last_idx + i],
            state.signal
        );
        assert!(
            approx_eq_float(state.histogram, expected_histogram[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected_histogram[last_idx + i],
            state.histogram
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_macd(
        "macd.csv",
        12,
        26,
        9,
    );
}

#[test]
fn generated_with_no_lookahead_fast16_slow36_signal12_ok() {
    generated_and_no_lookahead_macd(
        "macd_fastperiod-16_slowperiod-36_signalperiod-12.csv",
        16,
        36,
        12,
    );
}

#[test]
fn generated_with_no_lookahead_signal32_ok() {
    generated_and_no_lookahead_macd(
        "macd_signalperiod-32.csv",
        12,
        26,
        32,
    );
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
    let mut state = result.state;
    expect_err_overflow_or_ok_with!(state.update(Float::MIN + 5.0), |_| {
        assert!(state.macd.is_finite(), "Expected all values to be finite");
        assert!(state.signal.is_finite(), "Expected all values to be finite");
        assert!(
            state.histogram.is_finite(),
            "Expected all values to be finite"
        );
    });
}
