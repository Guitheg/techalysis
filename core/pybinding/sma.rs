use crate::features::sma::sma as core_sma;

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
pub fn sma(data: Vec<f64>, period: usize) -> PyResult<(usize, Vec<f64>)> {
    core_sma(&data, period)
        .map_err(|e| PyValueError::new_err(format!("Error computing SMA: {:?}", e)))
}
