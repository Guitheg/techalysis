use crate::oracle_test;
use crate::rust::tests_helper::{assert::assert_vec_close, oracle::read_fixture};
use technicalysis::{errors::TechnicalysisError, features::ema::ema};

oracle_test!(ema, |x: &[f64]| ema(x, 30, 2.0));

#[test]
fn test_ema_with_nan_must_fail() {
    let data = vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 3.0, 4.0, 2.0];
    let opt_in_time_period = 3;
    let result = ema(&data, opt_in_time_period, 2f64);
    assert!(matches!(result, Err(TechnicalysisError::UnexpectedNan)))
}
