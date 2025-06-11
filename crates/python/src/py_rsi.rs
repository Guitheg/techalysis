use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use techalysis::indicators::rsi::{rsi_into, RsiState};
use techalysis::types::Float;

#[pyclass(name = "RsiState")]
#[derive(Debug, Clone)]
pub struct PyRsiState {
    #[pyo3(get)]
    pub rsi: Float,
    #[pyo3(get)]
    pub prev_value: Float,
    #[pyo3(get)]
    pub avg_gain: Float,
    #[pyo3(get)]
    pub avg_loss: Float,
    #[pyo3(get)]
    pub period: usize,
}

#[pymethods]
impl PyRsiState {
    #[new]
    pub fn new(
        rsi: Float,
        prev_value: Float,
        avg_gain: Float,
        avg_loss: Float,
        period: usize,
    ) -> Self {
        PyRsiState {
            rsi,
            prev_value,
            avg_gain,
            avg_loss,
            period,
        }
    }

    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "RsiState(rsi={}, prev_value={}, avg_gain={}, avg_loss={}, period={})",
            self.rsi, self.prev_value, self.avg_gain, self.avg_loss, self.period
        )
    }
}

impl From<RsiState> for PyRsiState {
    fn from(state: RsiState) -> Self {
        PyRsiState {
            rsi: state.rsi,
            prev_value: state.prev_value,
            avg_gain: state.avg_gain,
            avg_loss: state.avg_loss,
            period: state.period,
        }
    }
}

impl From<PyRsiState> for RsiState {
    fn from(py_state: PyRsiState) -> Self {
        RsiState {
            rsi: py_state.rsi,
            prev_value: py_state.prev_value,
            avg_gain: py_state.avg_gain,
            avg_loss: py_state.avg_loss,
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn rsi(
    py: Python,
    data: numpy::PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyRsiState)> {
    let len: usize = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let rsi_state = py
            .allow_threads(|| rsi_into(input_slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((output.into_pyarray(py).into(), rsi_state.into()));
    } else {
        let output_array = PyArray1::<Float>::zeros(py, [len], false);
        let output_slice = unsafe { output_array.as_slice_mut()? };

        let rsi_state = rsi_into(input_slice, period, output_slice)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((output_array.into(), rsi_state.into()));
    }
}

#[pyfunction(signature = (new_value, rsi_state))]
pub(crate) fn rsi_next(new_value: Float, rsi_state: PyRsiState) -> PyResult<PyRsiState> {
    let mut state: RsiState = rsi_state.into();
    state.next(new_value).map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
    Ok(state.into())
}
