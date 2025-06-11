/*
    BSD 3-Clause License

    Copyright (c) 2025, Guillaume GOBIN (Guitheg)

    Redistribution and use in source and binary forms, with or without modification,
    are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation and/or
    other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors
    may be used to endorse or promote products derived from this software without
    specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
    THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/*
    List of contributors:
    - Guitheg: Initial implementation
*/

use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use techalysis::indicators::ema::{ema_into, period_to_alpha, EmaState};
use techalysis::traits::State;
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

impl From<PyEmaState> for EmaState {
    fn from(py_state: PyEmaState) -> Self {
        EmaState {
            ema: py_state.ema,
            period: py_state.period,
            alpha: py_state.alpha.unwrap_or(
                period_to_alpha(py_state.period, None).unwrap_or(2.0 / py_state.period as Float),
            ),
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
    let mut state: EmaState = ema_state.into();
    state
        .update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
    Ok(state.into())
}
