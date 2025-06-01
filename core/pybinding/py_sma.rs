use crate::indicators::sma as core_sma;
use numpy::IntoPyArray;
use pyo3::pyfunction;

#[pyfunction(signature = (data, window_size = 14))]
pub(crate) fn sma<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    window_size: usize,
) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
    let slice = data.as_slice()?;

    let vec = py
        .allow_threads(|| core_sma(slice, window_size))
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;
    Ok(vec.into_pyarray(py).into())
}
