use crate::indicators::ema as core_ema;
use numpy::IntoPyArray;
use pyo3::pyfunction;

#[pyfunction(signature = (data, window_size = 14, alpha = None))]
pub(crate) fn ema<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    window_size: usize,
    alpha: Option<f64>,
) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
    let slice = data.as_slice()?;

    let vec = py
        .allow_threads(|| core_ema(slice, window_size, alpha.into()))
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;
    Ok(vec.into_pyarray(py).into())
}
