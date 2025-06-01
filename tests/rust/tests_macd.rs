use technicalysis::{errors::TechnicalysisError, indicators::macd};

use crate::rust::tests_helper::generated::{assert_vec_eq_gen_data, load_generated_csv};

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
    let input: Vec<f64> = vec![];
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(
        matches!(output, Err(TechnicalysisError::InsufficientData)),
        "Got: {output:?}"
    );
}

#[test]
fn uniform_data() {
    let input: Vec<f64> = vec![100.0; 50];
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
    let input: Vec<f64> = (1..=20).map(|x| x as f64).collect();
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(
        matches!(output, Err(TechnicalysisError::InsufficientData)),
        "Got: {output:?}",
    );
}

#[test]
fn fast_greater_than_slow() {
    let input: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    let output = macd(&input, 30, 20, 9);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechnicalysisError::BadParam(_))));
}

#[test]
fn unexpected_nan() {
    let mut input: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    input[10] = f64::NAN; // Introduce NaN
    let output = macd(&input, 12, 26, 9);
    assert!(output.is_err());
    assert!(matches!(output, Err(TechnicalysisError::UnexpectedNan)));
}
