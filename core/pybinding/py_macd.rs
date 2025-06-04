use crate::indicators::macd::macd_into;

use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::{pyfunction, Py, PyResult};

#[pyfunction(signature = (data, fast_period = 12, slow_period = 26, signal_period = 9, release_gil = false))]
pub(crate) fn macd<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let len = data.len();
    let input_data = data.as_slice()?;

    if release_gil {
        let mut output_macd = vec![0.0; len];
        let mut output_signal = vec![0.0; len];
        let mut output_histogram = vec![0.0; len];

        py.allow_threads(|| {
            macd_into(
                input_data,
                fast_period,
                slow_period,
                signal_period,
                output_macd.as_mut_slice(),
                output_signal.as_mut_slice(),
                output_histogram.as_mut_slice(),
            )
        })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((
            output_macd.into_pyarray(py).into(),
            output_signal.into_pyarray(py).into(),
            output_histogram.into_pyarray(py).into(),
        ));
    } else {
        let py_array_macd = PyArray1::<f64>::zeros(py, [len], false);
        let output_macd_data = unsafe { py_array_macd.as_slice_mut()? };

        let py_array_signal = PyArray1::<f64>::zeros(py, [len], false);
        let output_signal_data = unsafe { py_array_signal.as_slice_mut()? };

        let py_array_histogram = PyArray1::<f64>::zeros(py, [len], false);
        let output_histogram_data = unsafe { py_array_histogram.as_slice_mut()? };

        macd_into(
            input_data,
            fast_period,
            slow_period,
            signal_period,
            output_macd_data,
            output_signal_data,
            output_histogram_data,
        )
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((
            py_array_macd.into(),
            py_array_signal.into(),
            py_array_histogram.into(),
        ));
    }
}
