use crate::oracle_test;
use crate::rust::tests_helper::{assert::assert_vec_close, oracle::read_fixture};
use technicalysis::features::rsi::rsi;

oracle_test!(rsi, |x: &[f64]| rsi(x, 14));
