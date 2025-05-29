use technicalysis::indicators::macd;

use crate::rust::tests_helper::generated::{assert_vec_eq_gen_data, load_generated_csv};

#[test]
fn generated() {
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
    assert!(&out.macd.len() == &input.len());
}
