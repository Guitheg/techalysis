use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use techalysis::indicators::bbands::{bbands_into, BBandsMA, BBandsState};

#[pyclass(name = "BBandsState")]
#[derive(Debug, Clone)]
pub struct PyBBandsState {
    #[pyo3(get)]
    pub upper: f64,
    #[pyo3(get)]
    pub middle: f64,
    #[pyo3(get)]
    pub lower: f64,
    #[pyo3(get)]
    pub mean_sma: f64,
    #[pyo3(get)]
    pub mean_sq: f64,
    #[pyo3(get)]
    pub window: Vec<f64>,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub std_up: f64,
    #[pyo3(get)]
    pub std_down: f64,
    #[pyo3(get)]
    pub ma_type: PyBBandsMA,
}

#[pyclass(name = "BBandsMA")]
#[derive(Debug, Clone, Copy)]
pub enum PyBBandsMA {
    SMA,
    EMA,
}

#[pymethods]
impl PyBBandsState {
    #[new]
    pub fn new(
        upper: f64,
        middle: f64,
        lower: f64,
        mean_sma: f64,
        mean_sq: f64,
        window: Vec<f64>,
        period: usize,
        std_up: f64,
        std_down: f64,
        ma_type: PyBBandsMA,
    ) -> Self {
        PyBBandsState {
            upper,
            middle,
            lower,
            mean_sma,
            mean_sq,
            window,
            period,
            std_up,
            std_down,
            ma_type,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "BBandsState(upper: {}, middle: {}, lower: {}, mean_sq: {}, window: {:?}, period: {}, std_up: {}, std_down: {}, ma_type: {:?})",
            self.upper, self.middle, self.lower, self.mean_sq, self.window, self.period, self.std_up, self.std_down, self.ma_type
        )
    }
}
impl From<BBandsState> for PyBBandsState {
    fn from(state: BBandsState) -> Self {
        PyBBandsState {
            upper: state.upper,
            middle: state.middle,
            lower: state.lower,
            mean_sma: state.sma,
            mean_sq: state.ma_sq,
            window: state.window.into(),
            period: state.period,
            std_up: state.std_up,
            std_down: state.std_down,
            ma_type: state.ma_type.into(),
        }
    }
}

impl From<PyBBandsState> for BBandsState {
    fn from(py_state: PyBBandsState) -> Self {
        BBandsState {
            upper: py_state.upper,
            middle: py_state.middle,
            lower: py_state.lower,
            sma: py_state.mean_sma,
            ma_sq: py_state.mean_sq,
            window: py_state.window.into(),
            period: py_state.period,
            std_up: py_state.std_up,
            std_down: py_state.std_down,
            ma_type: py_state.ma_type.into(),
        }
    }
}

impl From<PyBBandsMA> for BBandsMA {
    fn from(py_ma: PyBBandsMA) -> Self {
        match py_ma {
            PyBBandsMA::SMA => BBandsMA::SMA,
            PyBBandsMA::EMA => BBandsMA::EMA(None),
        }
    }
}

impl From<BBandsMA> for PyBBandsMA {
    fn from(ma: BBandsMA) -> Self {
        match ma {
            BBandsMA::SMA => PyBBandsMA::SMA,
            BBandsMA::EMA(_) => PyBBandsMA::EMA,
        }
    }
}

#[pyfunction(signature = (data, period = 20, std_up = 2.0, std_down = 2.0, ma_type = PyBBandsMA::SMA, release_gil = false))]
pub(crate) fn bbands(
    py: Python,
    data: PyReadonlyArray1<f64>,
    period: usize,
    std_up: f64,
    std_down: f64,
    ma_type: PyBBandsMA,
    release_gil: bool,
) -> PyResult<(
    Py<PyArray1<f64>>,
    Py<PyArray1<f64>>,
    Py<PyArray1<f64>>,
    PyBBandsState,
)> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output_upper = vec![0.0; len];
        let mut output_middle = vec![0.0; len];
        let mut output_lower = vec![0.0; len];

        let state = py
            .allow_threads(|| {
                bbands_into(
                    input_slice,
                    period,
                    std_up,
                    std_down,
                    ma_type.into(),
                    output_upper.as_mut_slice(),
                    output_middle.as_mut_slice(),
                    output_lower.as_mut_slice(),
                )
            })
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((
            output_upper.into_pyarray(py).into(),
            output_middle.into_pyarray(py).into(),
            output_lower.into_pyarray(py).into(),
            state.into(),
        ))
    } else {
        let py_out_upper = PyArray1::<f64>::zeros(py, [len], false);
        let py_out_upper_slice = unsafe { py_out_upper.as_slice_mut()? };

        let py_out_middle = PyArray1::<f64>::zeros(py, [len], false);
        let py_out_middle_slice = unsafe { py_out_middle.as_slice_mut()? };

        let py_out_lower = PyArray1::<f64>::zeros(py, [len], false);
        let py_out_lower_slice = unsafe { py_out_lower.as_slice_mut()? };

        let state = bbands_into(
            input_slice,
            period,
            std_up,
            std_down,
            ma_type.into(),
            py_out_upper_slice,
            py_out_middle_slice,
            py_out_lower_slice,
        )
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((
            py_out_upper.into(),
            py_out_middle.into(),
            py_out_lower.into(),
            state.into(),
        ))
    }
}

#[pyfunction(signature = (new_value, bbands_state))]
pub(crate) fn bbands_next(new_value: f64, bbands_state: PyBBandsState) -> PyResult<PyBBandsState> {
    let bbands_state: BBandsState = bbands_state.into();
    let bbands_state = bbands_state
        .next(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(bbands_state.into())
}
