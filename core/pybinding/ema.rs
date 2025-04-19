use crate::features::ema::ema as core_ema;
use numpy::{PyArray1, PyReadonlyArray1};

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
pub fn ema(
    py: Python,
    data: PyReadonlyArray1<f64>,
    period: usize,
    smoothing: f64,
) -> PyResult<Py<PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let result = core_ema(data_slice, period, smoothing)
        .map_err(|e| PyValueError::new_err(format!("Error computing EMA: {:?}", e)))?;
    let result_array = PyArray1::from_vec(py, result).to_owned();
    Ok(result_array.into())
}
