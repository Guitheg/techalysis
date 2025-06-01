use crate::indicators::macd as core_macd;

use numpy::{IntoPyArray, PyArray1};
use pyo3::{pyfunction, Py, PyResult};

#[pyfunction(signature = (data, fast_period = 12, slow_period = 26, signal_period = 9))]
pub(crate) fn macd<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let slice = data.as_slice()?;

    let output_macd = py
        .allow_threads(|| core_macd(slice, fast_period, slow_period, signal_period))
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

    Ok((
        output_macd.macd.into_pyarray(py).into(),
        output_macd.signal.into_pyarray(py).into(),
        output_macd.histogram.into_pyarray(py).into(),
    ))
}
