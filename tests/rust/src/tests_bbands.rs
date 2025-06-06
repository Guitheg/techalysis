use crate::helper::{
    assert::approx_eq_f64_custom,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use techalysis::indicators::bbands::bbands;

#[test]
fn generated() {
    let columns = load_generated_csv("bbands.csv").unwrap();

    let input = columns.get("close").unwrap();

    let upper = columns.get("upper").unwrap();
    let middle = columns.get("middle").unwrap();
    let lower = columns.get("lower").unwrap();

    let output = bbands(input, 20, 2.0, 2.0);
    assert!(output.is_ok());
    let result = output.unwrap();

    assert_vec_eq_gen_data(&result.upper_band, upper);
    assert_vec_eq_gen_data(&result.middle_band, middle);
    assert_vec_eq_gen_data(&result.lower_band, lower);
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

    let result = bbands(input_prev, 20, 2.0, 2.0).unwrap();

    assert_vec_eq_gen_data(&result.upper_band, &upper[0..last_idx]);
    assert_vec_eq_gen_data(&result.middle_band, &middle[0..last_idx]);
    assert_vec_eq_gen_data(&result.lower_band, &lower[0..last_idx]);

    let new_state = result.state.next(input[last_idx]).unwrap();
    assert!(
        approx_eq_f64_custom(new_state.upper, upper[last_idx], 1e-8),
        "Expected last value to be {}, but got {}",
        upper[last_idx],
        new_state.upper
    );
}

// TODO: IMPLEMENTS OTHER TESTS

// TODO: IMPLEMENTS proptest
// proptest! {
//     #[test]
//     fn proptest(
//        // TODO: DEFINE ARGS
//     ) {

//     }
// }
