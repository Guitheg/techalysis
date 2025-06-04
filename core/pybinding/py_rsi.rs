use crate::indicators::rsi::rsi_into;
use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::pyfunction;

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn rsi<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    period: usize,
    release_gil: bool,
) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
    let len = data.len();
    let slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        py.allow_threads(|| rsi_into(slice, period, output.as_mut_slice()))
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

        return Ok(output.into_pyarray(py).into());
    } else {
        let py_array_out = PyArray1::<f64>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };
        rsi_into(slice, period, py_array_ptr)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

        return Ok(py_array_out.into());
    }
}
