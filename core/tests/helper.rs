use std::fs::File;

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

#[allow(dead_code)]
pub fn read_fixture(name: &str) -> (Vec<f64>, Vec<f64>) {
    let file = File::open(format!("core/tests/data/oracle/{}.csv", name))
        .unwrap_or_else(|e| panic!("Failed to find {e}"));
    let mut rdr = csv::Reader::from_reader(file);

    let mut x = Vec::new();
    let mut y = Vec::new();
    for rec in rdr.records() {
        let rec = rec.unwrap();
        x.push(rec[0].parse::<f64>().unwrap());
        y.push(rec[1].parse::<f64>().unwrap());
    }
    (x, y)
}

#[allow(dead_code)]
pub fn assert_vec_close(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    assert_vec_float_eq!(a, b, 1e-9);
}
