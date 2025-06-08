use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use proptest::{collection::vec, prelude::*};
use techalysis::{
    errors::TechalysisError,
    indicators::sma::{sma, SmaResult, SmaState},
    types::Float,
};

#[test]
fn generated() {
    let columns = load_generated_csv("sma.csv").unwrap();
    let input = columns.get("close").unwrap();
    let output = sma(input, 30);
    assert!(output.is_ok());
    let out = output.unwrap();
    let expected = columns.get("out").unwrap();
    assert_vec_eq_gen_data(&out.values, expected);
    assert!(out.values.len() == input.len());
}

#[test]
fn no_lookahead() {
    let columns = load_generated_csv("sma.csv").unwrap();
    let input = columns.get("close").unwrap();
    let expected = columns.get("out").unwrap();
    // Check that the state given by : 'out_sma, state = sma(input[..-1])' is equal to sma_next(input[-1], state)
    let output = sma(&input[0..input.len() - 1], 30);
    assert!(output.is_ok());
    let result = output.unwrap();
    assert_vec_eq_gen_data(&result.values, &expected[0..&expected.len() - 1]);

    let new_output = result.state.next(input[input.len() - 1]);
    assert!(
        new_output.is_ok(),
        "Expected next state to be Ok, but got an error: {new_output:?}",
    );
    let new_state = new_output.unwrap();
    assert!(
        approx_eq_float(new_state.sma, expected[expected.len() - 1], 1e-8),
        "Expected last value to be {}, but got {}",
        expected[expected.len() - 1],
        new_state.sma
    );
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
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(sma(&data, period), |result: SmaResult| {
        assert!(
            result.values.iter().skip(period).all(|v| v.is_finite()),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;
    let result = sma(&data, period).unwrap();
    expect_err_overflow_or_ok_with!(result.state.next(Float::MIN + 5.0), |state: SmaState| {
        assert!(state.sma.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn invalid_period_lower_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 0);
    assert!(result.is_err());
    if let Err(TechalysisError::BadParam(msg)) = result {
        assert!(msg.contains("between 2 and 100000"));
    }
}

#[test]
fn period_higher_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 3);
    assert!(result.is_ok());
    let out = result.unwrap().values;
    assert!(out[2] == 2.0);
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let result = sma(&data, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let result = sma(&data, 3);
    assert!(
        result.is_err(),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalysisError::DataNonFinite(_))));
}

#[test]
fn insufficient_data_err() {
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 4);
    assert!(matches!(result, Err(TechalysisError::InsufficientData)));
}

#[test]
fn period_1() {
    let data = &[10.0, 11.0, 10.0, 10.0, 12.0];
    let period = 1;
    let result = sma(data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalysisError::BadParam(_))));
}

fn slow_sma(data: &[Float], window: usize) -> Vec<Float> {
    let mut out = vec![Float::NAN; data.len()];
    if window == 0 || window > data.len() {
        return out;
    }

    for i in window - 1..data.len() {
        let slice = &data[i + 1 - window..=i];
        out[i] = slice.iter().sum::<Float>() / window as Float;
    }
    out
}

proptest! {
    #[test]
    fn proptest(
        input  in vec(-1e12f64..1e12, 2..200),
        window in 2usize..50
    ) {
        prop_assume!(window <= input.len());
        let has_nan = input.iter().all(|v| v.is_nan());
        let out = sma(&input, window);

        if has_nan {
            prop_assert!(out.is_err());
            prop_assert!(matches!(out, Err(TechalysisError::DataNonFinite(_))));
        } else {
            let out = out.unwrap().values;
            prop_assert_eq!(out.len(), input.len());
            prop_assert!(out[..window-1].iter().all(|v| v.is_nan()));

            // Definition
            let slow = slow_sma(&input, window);
            for (o, expect) in out.iter().zip(slow) {
                if o.is_nan() || expect.is_nan() {
                    prop_assert!(o.is_nan() && expect.is_nan());
                } else {
                    prop_assert!(approx_eq_float(*o, expect, 1e-8));
                }
            }

            // Linearity
            let k = 4.3;
            let scaled_input: Vec<_> = input.iter().map(|v| v * k).collect();
            let scaled_fast = sma(&scaled_input, window).unwrap().values;

            for (o, scaled) in out.iter().zip(scaled_fast) {
                if o.is_nan() {
                    prop_assert!(scaled.is_nan());
                } else {
                    prop_assert!(approx_eq_float(scaled, o * k, 1e-8));
                }
            }
        }
    }
}
