use std::{collections::HashMap, fs::File, path::Path};

use csv::ReaderBuilder;

use crate::assert_vec_float_eq;

pub const GENERATED_CSV_DIR: &str = "../../tests/data/generated";
pub const GENERATED_PRECISION: f64 = 1e-8;

pub fn assert_vec_eq_gen_data(expected: &[f64], got: &[f64]) {
    assert_eq!(expected.len(), got.len());
    assert_vec_float_eq!(expected, got, GENERATED_PRECISION);
}

pub fn load_generated_csv(file_name: &str) -> Result<HashMap<String, Vec<f64>>, csv::Error> {
    let mut rdr =
        ReaderBuilder::new().from_reader(File::open(Path::new(GENERATED_CSV_DIR).join(file_name))?);
    let headers = rdr.headers()?.clone();
    let mut columns: HashMap<String, Vec<f64>> =
        headers.iter().map(|h| (h.to_string(), vec![])).collect();

    for record in rdr.records() {
        let record = record?;
        for (header, value) in headers.iter().zip(record.iter()) {
            let parsed = value.parse::<f64>().unwrap_or(f64::NAN);
            columns.get_mut(header).unwrap().push(parsed);
        }
    }
    Ok(columns)
}
