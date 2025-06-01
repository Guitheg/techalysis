use std::num::FpCategory;

pub fn approx_eq_f64_custom(a: f64, b: f64, eps_factor: f64) -> bool {
    let ulp_step = ulp_at(b);
    let min_tol: f64 = (1e-1_f64).max(ulp_step);
    let diff = (a - b).abs();
    let relative_tol = (eps_factor * b.abs().max(1.0)).min(min_tol);
    diff < relative_tol
}

pub fn approx_eq_f64(a: f64, b: f64) -> bool {
    let eps: f64 = 1e-9;
    approx_eq_f64_custom(a, b, eps)
}

#[macro_export]
macro_rules! assert_vec_float_eq {
    ($a:expr, $b:expr, $epsilon:expr) => {{
        use $crate::rust::tests_helper::assert::approx_eq_f64_custom;
        for (i, (x, y)) in $a.iter().zip($b.iter()).enumerate() {
            if x.is_nan() && y.is_nan() {
                continue;
            }
            assert!(
                approx_eq_f64_custom(*x, *y, $epsilon),
                "[{i}] Expected: {}, got: {} (epsilon: {})",
                x,
                y,
                $epsilon
            );
        }
    }};
}

pub fn assert_vec_close(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    assert_vec_float_eq!(a, b, 1e-9);
}

fn ulp_at(x: f64) -> f64 {
    match x.classify() {
        FpCategory::Nan | FpCategory::Infinite => f64::NAN,
        _ => {
            let next = f64::from_bits(x.to_bits() + 1);
            next - x
        }
    }
}

mod test {
    use crate::rust::tests_helper::assert::approx_eq_f64;

    #[test]
    fn test_1_success() {
        assert!(approx_eq_f64(6_238_020.570_828_672, 6_238_020.570_942_126));
    }

    #[test]
    fn test_2_success() {
        assert!(approx_eq_f64(623_802_300.570_828_7, 623_802_300.570_942_2));
    }

    #[test]
    fn test_3_success() {
        assert!(approx_eq_f64(623_802_300_001.570_8, 623_802_300_001.570_9));
    }

    #[test]
    fn test_4_success() {
        assert!(approx_eq_f64(6_238_023_000_010.571, 6_238_023_000_010.571));
    }

    #[test]
    fn test_5_success() {
        assert!(approx_eq_f64(623_802_300_001_000.6, 623_802_300_001_000.6));
    }

    #[test]
    fn test_6_success() {
        assert!(approx_eq_f64(0.5708286715679999, 0.5708286717));
    }

    #[test]
    fn test_7_success() {
        assert!(approx_eq_f64(0.00000000333333333, 0.00000001_f64 / 3_f64));
    }

    #[test]
    fn test_8_success() {
        assert!(approx_eq_f64(0.0, 0.0));
    }

    #[test]
    fn test_9_success() {
        assert!(approx_eq_f64(1e-10, 0.0));
    }

    #[test]
    fn test_10_success() {
        assert!(approx_eq_f64(0.0, 1e-10));
    }

    #[test]
    fn test_1_failure() {
        assert!(!approx_eq_f64(6_238_020_000_000.571, 6_238_020_000_001.571));
    }

    #[test]
    fn test_2_failure() {
        assert!(!approx_eq_f64(623_802_300_001_000.6, 623_802_300_001_001.6));
    }

    #[test]
    fn test_3_failure() {
        assert!(!approx_eq_f64(
            6238023000010050.57082867156714201,
            6238023000010050.47094212621450424
        ));
    }

    #[test]
    fn test_4_failure() {
        assert!(!approx_eq_f64(
            1.570_828_671_567_142,
            1.570_942_126_214_504_2
        ));
    }

    #[test]
    fn test_5_failure() {
        assert!(!approx_eq_f64(0.12, 0.03));
    }
}
