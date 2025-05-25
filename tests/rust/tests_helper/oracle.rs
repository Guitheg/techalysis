use std::fs::File;

pub fn read_fixture(name: &str) -> (Vec<f64>, Vec<Vec<f64>>) {
    let file = File::open(format!("tests/data/{}.csv", name))
        .unwrap_or_else(|e| panic!("Failed to find {e}"));
    let mut rdr = csv::Reader::from_reader(file);

    let mut x = Vec::new();
    let mut channels: Vec<Vec<f64>> = Vec::new();
    for rec in rdr.records() {
        let rec = rec.unwrap();
        x.push(rec[0].parse::<f64>().unwrap());
        for (i, value) in rec.iter().enumerate().skip(1) {
            if channels.len() < i {
                channels.push(Vec::new());
            }
            channels[i - 1].push(value.parse::<f64>().unwrap());
        }
    }
    (x, channels)
}

#[macro_export]
macro_rules! oracle_test {
    ($name: ident, $f: expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $name _with_oracle>]() {
                let (input, expected_channels) = read_fixture(concat!("oracle/", stringify!($name)));
                let output = $f(&input);
                assert!(output.is_ok());
                let output = output.unwrap();
                assert_eq!(output.len(), expected_channels.len());
                for (output_channel, expected_channel) in output.iter().zip(expected_channels.iter()) {
                    assert_vec_close(output_channel, expected_channel);
                }
            }
        }
    };
}
