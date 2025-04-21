#[macro_export]
macro_rules! assert_vec_float_eq {
    ($a:expr, $b:expr, $epsilon:expr) => {{
        for (i, (x, y)) in $a.iter().zip($b.iter()).enumerate() {
            if x.is_nan() && y.is_nan() {
                continue;
            }
            assert!(
                (x - y).abs() < $epsilon,
                "Failed at index {} -> {} != {} (epsilon: {})",
                i,
                x,
                y,
                $epsilon
            );
        }
    }};
}
