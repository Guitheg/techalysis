use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use technicalysis::indicators::macd::{macd_into, macd_next as core_macd_next, MacdState};

#[pyclass(name = "MacdState")]
#[derive(Debug, Clone)]
pub struct PyMacdState {
    #[pyo3(get)]
    pub macd: f64,
    #[pyo3(get)]
    pub signal: f64,
    #[pyo3(get)]
    pub histogram: f64,
    #[pyo3(get)]
    pub fast_ema: f64,
    #[pyo3(get)]
    pub slow_ema: f64,
    #[pyo3(get)]
    pub fast_period: usize,
    #[pyo3(get)]
    pub slow_period: usize,
    #[pyo3(get)]
    pub signal_period: usize,
}
#[pymethods]
impl PyMacdState {
    #[new]
    pub fn new(
        macd: f64,
        signal: f64,
        histogram: f64,
        fast_ema: f64,
        slow_ema: f64,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Self {
        PyMacdState {
            macd,
            signal,
            histogram,
            fast_ema,
            slow_ema,
            fast_period,
            slow_period,
            signal_period,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "MacdState(macd={}, signal={}, histogram={}, fast_ema={}, slow_ema={}, fast_period={}, slow_period={}, signal_period={})",
            self.macd, self.signal, self.histogram, self.fast_ema, self.slow_ema, self.fast_period, self.slow_period, self.signal_period
        )
    }
}
impl From<MacdState> for PyMacdState {
    fn from(state: MacdState) -> Self {
        PyMacdState {
            macd: state.macd,
            signal: state.signal,
            histogram: state.histogram,
            fast_ema: state.fast_ema,
            slow_ema: state.slow_ema,
            fast_period: state.fast_period,
            slow_period: state.slow_period,
            signal_period: state.signal_period,
        }
    }
}

#[pyfunction(signature = (data, fast_period = 12, slow_period = 26, signal_period = 9, release_gil = false))]
pub(crate) fn macd(
    py: Python,
    data: PyReadonlyArray1<f64>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    release_gil: bool,
) -> PyResult<(
    Py<PyArray1<f64>>,
    Py<PyArray1<f64>>,
    Py<PyArray1<f64>>,
    PyMacdState,
)> {
    let len = data.len();
    let input_data = data.as_slice()?;

    if release_gil {
        let mut output_macd = vec![0.0; len];
        let mut output_signal = vec![0.0; len];
        let mut output_histogram = vec![0.0; len];

        let macd_state = py
            .allow_threads(|| {
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
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((
            output_macd.into_pyarray(py).into(),
            output_signal.into_pyarray(py).into(),
            output_histogram.into_pyarray(py).into(),
            macd_state.into(),
        ));
    } else {
        let py_array_macd = PyArray1::<f64>::zeros(py, [len], false);
        let output_macd_data = unsafe { py_array_macd.as_slice_mut()? };

        let py_array_signal = PyArray1::<f64>::zeros(py, [len], false);
        let output_signal_data = unsafe { py_array_signal.as_slice_mut()? };

        let py_array_histogram = PyArray1::<f64>::zeros(py, [len], false);
        let output_histogram_data = unsafe { py_array_histogram.as_slice_mut()? };

        let macd_state = macd_into(
            input_data,
            fast_period,
            slow_period,
            signal_period,
            output_macd_data,
            output_signal_data,
            output_histogram_data,
        )
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((
            py_array_macd.into(),
            py_array_signal.into(),
            py_array_histogram.into(),
            macd_state.into(),
        ));
    }
}

#[pyfunction(signature = (new_value, macd_state,))]
pub(crate) fn macd_next(new_value: f64, macd_state: PyMacdState) -> PyResult<PyMacdState> {
    let state = core_macd_next(
        new_value,
        macd_state.fast_ema,
        macd_state.slow_ema,
        macd_state.signal,
        macd_state.fast_period,
        macd_state.slow_period,
        macd_state.signal_period,
    )
    .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
    Ok(state.into())
}
