use std::fs::File;

pub fn read_fixture(name: &str) -> (Vec<f64>, Vec<f64>) {
    let file = File::open(format!("tests/data/{}.csv", name))
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

#[macro_export]
macro_rules! oracle_test {
    ($name: ident, $f: expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $name _with_oracle>]() {
                let (input, expected) = read_fixture(concat!("oracle/", stringify!($name)));
                let output = $f(&input);
                assert!(output.is_ok());
                assert_vec_close(&output.unwrap(), &expected);
            }
        }
    };
}
