use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use techalysis::indicators::ema::{ema_into, ema_next as core_ema_next, EmaState};
use techalysis::types::Float;

#[derive(Debug, Clone)]
#[pyclass(name = "EmaState", module = "techalysis._core")]
pub struct PyEmaState {
    #[pyo3(get)]
    pub ema: Float,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub alpha: Option<Float>,
}
#[pymethods]
impl PyEmaState {
    #[new]
    pub fn new(ema: Float, period: usize, alpha: Option<Float>) -> Self {
        PyEmaState { ema, period, alpha }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "EmaState(ema={}, period={}, alpha={:?})",
            self.ema, self.period, self.alpha
        )
    }
}
impl From<EmaState> for PyEmaState {
    fn from(state: EmaState) -> Self {
        PyEmaState {
            ema: state.ema,
            period: state.period,
            alpha: state.alpha.into(),
        }
    }
}

#[pyfunction(signature = (data, period = 14, alpha = None, release_gil = false))]
pub(crate) fn ema(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    alpha: Option<Float>,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyEmaState)> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let state = py
            .allow_threads(|| ema_into(input_slice, period, alpha, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let output_array = PyArray1::<Float>::zeros(py, [len], false);
        let output_slice = unsafe { output_array.as_slice_mut()? };
        let state = ema_into(input_slice, period, alpha, output_slice)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok((output_array.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, ema_state))]
pub(crate) fn ema_next(new_value: Float, ema_state: PyEmaState) -> PyResult<PyEmaState> {
    let state = core_ema_next(new_value, ema_state.ema, ema_state.period, ema_state.alpha)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
    Ok(state.into())
}
