use crate::{
    expect_err_overflow_or_ok_with,
    helper::{
        assert::approx_eq_float,
        generated::{assert_vec_eq_gen_data, load_generated_csv},
    },
};
use proptest::{collection::vec, prelude::*};
use techalib::{
    errors::TechalibError,
    indicators::ema::{ema, ema_into, period_to_alpha, EmaResult},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_ema(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let input = columns.get("close").unwrap();

    let len = input.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let input_prev = &input[0..last_idx];

    let output = ema(input_prev, period, None);
    assert!(
        output.is_ok(),
        "Failed to calculate EMA: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        new_state.update(input[last_idx + i]).unwrap();
        assert!(
            approx_eq_float(new_state.ema, expected[last_idx + i], 1e-8),
            "Next expected {}, but got {}",
            expected[last_idx + i],
            new_state.ema
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_ema("ema.csv", 30);
}

#[test]
fn invalid_period_lower_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 0, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}

#[test]
fn period_higher_bound() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 3, None);
    assert!(result.is_ok());
    let out = result.unwrap().values;
    assert!(out[2] == 2.0);
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN];
    let result = ema(&data, 3, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, 3.0, Float::INFINITY, 5.0, 6.0, 7.0];
    let result = ema(&data, 3, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn insufficient_data_err() {
    let data = vec![1.0, 2.0, 3.0];
    let result = ema(&data, 4, None);
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn period_1() {
    let data = &[10.0, 11.0, 10.0, 10.0, 12.0];
    let period = 1;
    let result = ema(data, period, None);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}

#[test]
fn test_period_to_alpha() {
    assert_eq!(period_to_alpha(10, None).unwrap(), 0.18181818181818182);
    assert_eq!(period_to_alpha(10, Some(2.0)).unwrap(), 0.18181818181818182);
    assert!(period_to_alpha(0, None).is_err());
    assert!(period_to_alpha(10, Some(0.0)).is_err());
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
    expect_err_overflow_or_ok_with!(ema(&data, period, None), |result: EmaResult| {
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
    let result = ema(&data, period, None).unwrap();
    let mut state = result.state;
    expect_err_overflow_or_ok_with!(state.update(Float::MIN + 5.0), |_| {
        assert!(state.ema.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn different_length_input_output_err() {
    let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut output = vec![0.0; 3];
    let period = 3;
    let result = ema_into(&input, period, None, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}

proptest! {
    #[test]
    fn proptest(
        input  in vec(-1e12 as Float..1e12 as Float, 2..200),
        window in 2usize..50
    ) {
        prop_assume!(window <= input.len());
        let has_nan = input.iter().all(|v| v.is_nan());
        let out = ema(&input, window, None);

        if has_nan {
            prop_assert!(out.is_err());
            prop_assert!(matches!(out, Err(TechalibError::DataNonFinite(_))));
        } else {
            let out = out.unwrap().values;

            prop_assert_eq!(out.len(), input.len());
            prop_assert!(out[..window-1].iter().all(|v| v.is_nan()));

            let k = 7.0 as Float;
            let scaled_input: Vec<_> = input.iter().map(|v| v*k).collect();
            let scaled_fast = ema(&scaled_input, window, None).unwrap().values;

            for (orig, scaled) in out.iter().zip(&scaled_fast) {
                if orig.is_nan() {
                    prop_assert!(scaled.is_nan());
                } else {
                    let target = *orig * k;
                    prop_assert!(approx_eq_float(target, *scaled, 1e-8),
                        "scaling fails: ema={}, k*ema={}, got={}", orig, target, scaled);
                }
            }
        }
    }
}
