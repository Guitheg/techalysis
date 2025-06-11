use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use techalysis::indicators::dema::{dema_into, DemaState};
use techalysis::indicators::ema::period_to_alpha;
use techalysis::types::Float;

#[pyclass(name = "DemaState")]
#[derive(Debug, Clone)]
pub struct PyDemaState {
    #[pyo3(get)]
    pub dema: Float,
    #[pyo3(get)]
    pub ema_1: Float,
    #[pyo3(get)]
    pub ema_2: Float,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub alpha: Option<Float>,
}
#[pymethods]
impl PyDemaState {
    #[new]
    pub fn new(
        dema: Float,
        ema_1: Float,
        ema_2: Float,
        period: usize,
        alpha: Option<Float>,
    ) -> Self {
        PyDemaState {
            dema,
            ema_1,
            ema_2,
            period,
            alpha,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "DemaState(dema={}, ema_1={}, ema_2={}, period={}, alpha={:?})",
            self.dema, self.ema_1, self.ema_2, self.period, self.alpha
        )
    }
}
impl From<DemaState> for PyDemaState {
    fn from(state: DemaState) -> Self {
        PyDemaState {
            dema: state.dema,
            ema_1: state.ema_1,
            ema_2: state.ema_2,
            period: state.period,
            alpha: state.alpha.into(),
        }
    }
}

impl From<PyDemaState> for DemaState {
    fn from(py_state: PyDemaState) -> Self {
        DemaState {
            dema: py_state.dema,
            ema_1: py_state.ema_1,
            ema_2: py_state.ema_2,
            period: py_state.period,
            alpha: py_state.alpha.unwrap_or(
                period_to_alpha(py_state.period, None).unwrap_or(2.0 / py_state.period as Float),
            ),
        }
    }
}

#[pyfunction(signature = (data, period = 14, alpha = None, release_gil = false))]
pub(crate) fn dema(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    alpha: Option<Float>,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyDemaState)> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| dema_into(input_slice, period, alpha, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = dema_into(input_slice, period, alpha, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, dema_state))]
pub(crate) fn dema_next(new_value: Float, dema_state: PyDemaState) -> PyResult<PyDemaState> {
    let mut dema_state: DemaState = dema_state.into();
    dema_state
        .next(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(dema_state.into())
}
