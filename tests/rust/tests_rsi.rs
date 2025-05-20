use crate::oracle_test;
use crate::rust::tests_helper::{assert::assert_vec_close, oracle::read_fixture};
use proptest::{collection::vec, prelude::*};
use technicalysis::{errors::TechnicalysisError, features::rsi::rsi};

oracle_test!(rsi, |x: &[f64]| rsi(x, 14));
