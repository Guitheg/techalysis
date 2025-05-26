use technicalysis::indicators::macd;

use crate::oracle_test;
use crate::rust::tests_helper::{assert::assert_vec_close, oracle::read_fixture};

oracle_test!(macd, |x: &[f64]| {
    macd(x, 12, 26, 9).map(|result| vec![result.macd, result.signal, result.histogram])
});
