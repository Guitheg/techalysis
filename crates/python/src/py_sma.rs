use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::pymethods;
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, Py, PyResult, Python};
use techalysis::indicators::sma::{sma_into, SmaState};

#[pyclass(name = "SmaState")]
#[derive(Debug, Clone)]
pub struct PySmaState {
    #[pyo3(get)]
    pub sma: f64,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub window: Vec<f64>,
}
#[pymethods]
impl PySmaState {
    #[new]
    pub fn new(sma: f64, period: usize, window: Vec<f64>) -> Self {
        PySmaState {
            sma,
            period,
            window,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!("SmaState(sma={}, period={})", self.sma, self.period)
    }
}
impl From<SmaState> for PySmaState {
    fn from(state: SmaState) -> Self {
        PySmaState {
            sma: state.sma,
            period: state.period,
            window: state.window.into(),
        }
    }
}

impl From<PySmaState> for SmaState {
    fn from(py_state: PySmaState) -> Self {
        SmaState {
            sma: py_state.sma,
            period: py_state.period,
            window: py_state.window.into(),
        }
    }
}

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn sma(
    py: Python,
    data: PyReadonlyArray1<f64>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<f64>>, PySmaState)> {
    let len = data.len();
    let slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let state = py
            .allow_threads(|| sma_into(slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<f64>::zeros(py, [slice.len()], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = sma_into(slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, sma_state))]
pub(crate) fn sma_next(new_value: f64, sma_state: PySmaState) -> PyResult<PySmaState> {
    let sma_state: SmaState = sma_state.into();
    let sma_state = sma_state
        .next(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(sma_state.into())
}
