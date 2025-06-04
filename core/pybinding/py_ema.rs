use crate::indicators::ema::core_ema;
use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyfunction};

#[pyfunction(signature = (data, period = 14, alpha = None, release_gil = false))]
pub(crate) fn ema<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    period: usize,
    alpha: Option<f64>,
    release_gil: bool,
) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        py.allow_threads(|| core_ema(input_slice, period, alpha, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        return Ok(output.into_pyarray(py).into());
    } else {
        let output_array = PyArray1::<f64>::zeros(py, [len], false);
        let output_slice = unsafe { output_array.as_slice_mut()? };
        core_ema(input_slice, period, alpha, output_slice)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        return Ok(output_array.into());
    }
}
