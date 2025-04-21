use crate::features::sma::sma as core_sma;
use numpy::{PyArray1, PyReadonlyArray1};

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction(signature = (data, period, handle_nan = false))]
pub fn sma(
    py: Python,
    data: PyReadonlyArray1<f64>,
    period: usize,
    handle_nan: bool,
) -> PyResult<Py<PyArray1<f64>>> {
    let data_slice = data.as_slice()?;
    let result = core_sma(data_slice, period, handle_nan)
        .map_err(|e| PyValueError::new_err(format!("Error computing SMA: {:?}", e)))?;
    let result_array = PyArray1::from_vec(py, result).to_owned();
    Ok(result_array.into())
}
