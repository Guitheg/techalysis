use std::num::FpCategory;
use techalib::types::Float;

pub fn approx_eq_float(a: Float, b: Float, eps_factor: Float) -> bool {
    let ulp_step = ulp_at(b);
    let min_tol: Float = (1e-1_f64).max(ulp_step);
    let diff = (a - b).abs();
    let relative_tol = (eps_factor * b.abs().max(1.0)).min(min_tol);
    diff < relative_tol
}

#[macro_export]
macro_rules! assert_vec_float_eq {
    ($expected:expr, $got:expr, $epsilon:expr) => {{
        use $crate::helper::assert::approx_eq_float;
        for (i, (x, y)) in $expected.iter().zip($got.iter()).enumerate() {
            if x.is_nan() && y.is_nan() {
                continue;
            }
            assert!(
                approx_eq_float(*x, *y, $epsilon),
                "[{i}] Expected: {}, got: {} (epsilon: {})",
                x,
                y,
                $epsilon
            );
        }
    }};
}

pub fn assert_vec_close(expected: &[Float], got: &[Float]) {
    assert_eq!(expected.len(), got.len());
    assert_vec_float_eq!(expected, got, 1e-9);
}

fn ulp_at(x: Float) -> Float {
    match x.classify() {
        FpCategory::Nan | FpCategory::Infinite => Float::NAN,
        _ => {
            let next = Float::from_bits(x.to_bits() + 1);
            next - x
        }
    }
}

mod test {
    use crate::helper::assert::approx_eq_float;

    #[test]
    fn test_1_success() {
        assert!(approx_eq_float(
            6_238_020.570_828_672,
            6_238_020.570_942_126,
            1e-8
        ));
    }

    #[test]
    fn test_2_success() {
        assert!(approx_eq_float(
            623_802_300.570_828_7,
            623_802_300.570_942_2,
            1e-8
        ));
    }

    #[test]
    fn test_3_success() {
        assert!(approx_eq_float(
            623_802_300_001.570_8,
            623_802_300_001.570_9,
            1e-8
        ));
    }

    #[test]
    fn test_4_success() {
        assert!(approx_eq_float(
            6_238_023_000_010.571,
            6_238_023_000_010.571,
            1e-8
        ));
    }

    #[test]
    fn test_5_success() {
        assert!(approx_eq_float(
            623_802_300_001_000.6,
            623_802_300_001_000.6,
            1e-8
        ));
    }

    #[test]
    fn test_6_success() {
        assert!(approx_eq_float(0.5708286715679999, 0.5708286717, 1e-8));
    }

    #[test]
    fn test_7_success() {
        assert!(approx_eq_float(
            0.00000000333333333,
            0.00000001_f64 / 3_f64,
            1e-8
        ));
    }

    #[test]
    fn test_8_success() {
        assert!(approx_eq_float(0.0, 0.0, 1e-8));
    }

    #[test]
    fn test_9_success() {
        assert!(approx_eq_float(1e-10, 0.0, 1e-8));
    }

    #[test]
    fn test_10_success() {
        assert!(approx_eq_float(0.0, 1e-10, 1e-8));
    }

    #[test]
    fn test_1_failure() {
        assert!(!approx_eq_float(
            6_238_020_000_000.571,
            6_238_020_000_001.571,
            1e-8
        ));
    }

    #[test]
    fn test_2_failure() {
        assert!(!approx_eq_float(
            623_802_300_001_000.6,
            623_802_300_001_001.6,
            1e-8
        ));
    }

    #[test]
    fn test_3_failure() {
        assert!(!approx_eq_float(
            6238023000010050.57082867156714201,
            6238023000010050.47094212621450424,
            1e-8
        ));
    }

    #[test]
    fn test_4_failure() {
        assert!(!approx_eq_float(
            1.570_828_671_567_142,
            1.570_942_126_214_504_2,
            1e-8
        ));
    }

    #[test]
    fn test_5_failure() {
        assert!(!approx_eq_float(0.12, 0.03, 1e-8));
    }
}
