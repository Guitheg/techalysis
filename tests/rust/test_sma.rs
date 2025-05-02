use technicalysis::errors::TechnicalysisError;
use technicalysis::features::sma::sma;
use technicalysis::oracle_test;
use technicalysis::tests_helper::{assert::assert_vec_close, oracle::read_fixture};

oracle_test!(sma, |x: &[f64]| sma(x, 30));

#[test]
fn test_invalid_period() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 0);
    assert!(result.is_err());
    if let Err(TechnicalysisError::BadParam(msg)) = result {
        assert!(msg.contains("between 2 and 100000"));
    }
}

#[test]
fn test_insufficient_data() {
    let data = vec![1.0, 2.0];
    let result = sma(&data, 3);
    assert!(matches!(result, Err(TechnicalysisError::InsufficientData)));
}
