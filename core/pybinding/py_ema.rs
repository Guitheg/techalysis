use crate::indicators::ema::core_ema;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::pyfunction;

#[pyfunction(signature = (data, window_size = 14, alpha = None))]
pub(crate) fn ema<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    window_size: usize,
    alpha: Option<f64>,
) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
    let slice = data.as_slice()?;

    let py_array_out = unsafe { PyArray1::<f64>::new(py, [slice.len()], false) };
    let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

    py.allow_threads(|| core_ema(slice, window_size, alpha.into(), py_array_ptr))
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

    Ok(py_array_out.into())
}
