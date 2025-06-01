use crate::indicators::macd::core_macd;

use numpy::{PyArray1, PyArrayMethods};
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

    let py_array_macd = unsafe { PyArray1::<f64>::new(py, [slice.len()], false) };
    let py_macd_ptr = unsafe { py_array_macd.as_slice_mut()? };

    let py_array_signal = unsafe { PyArray1::<f64>::new(py, [slice.len()], false) };
    let py_signal_ptr = unsafe { py_array_signal.as_slice_mut()? };

    let py_array_histogram = unsafe { PyArray1::<f64>::new(py, [slice.len()], false) };
    let py_histogram_ptr = unsafe { py_array_histogram.as_slice_mut()? };

    py.allow_threads(|| {
        core_macd(
            slice,
            fast_period,
            slow_period,
            signal_period,
            py_macd_ptr,
            py_signal_ptr,
            py_histogram_ptr,
        )
    })
    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

    Ok((
        py_array_macd.into(),
        py_array_signal.into(),
        py_array_histogram.into(),
    ))
}
