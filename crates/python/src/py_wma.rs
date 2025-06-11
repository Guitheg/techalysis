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
use pyo3::pymethods;
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, Py, PyResult, Python};
use techalysis::indicators::wma::{wma_into, WmaState};
use techalysis::traits::State;
use techalysis::types::Float;

#[pyclass(name = "WmaState")]
#[derive(Debug, Clone)]
pub struct PyWmaState {
    #[pyo3(get)]
    pub wma: Float,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub period_sub: Float,
    #[pyo3(get)]
    pub period_sum: Float,
    #[pyo3(get)]
    pub window: Vec<Float>,
}
#[pymethods]
impl PyWmaState {
    #[new]
    pub fn new(
        wma: Float,
        period: usize,
        period_sub: Float,
        period_sum: Float,
        window: Vec<Float>,
    ) -> Self {
        PyWmaState {
            wma,
            period,
            period_sub,
            period_sum,
            window,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!("WmaState(wma={}, period={})", self.wma, self.period)
    }
}
impl From<WmaState> for PyWmaState {
    fn from(state: WmaState) -> Self {
        PyWmaState {
            wma: state.wma,
            period: state.period,
            period_sub: state.period_sub,
            period_sum: state.period_sum,
            window: state.last_window.into(),
        }
    }
}

impl From<PyWmaState> for WmaState {
    fn from(py_state: PyWmaState) -> Self {
        WmaState {
            wma: py_state.wma,
            period: py_state.period,
            period_sub: py_state.period_sub,
            period_sum: py_state.period_sum,
            last_window: py_state.window.into(),
        }
    }
}

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn wma(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyWmaState)> {
    let len = data.len();
    let slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let state = py
            .allow_threads(|| wma_into(slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [slice.len()], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = wma_into(slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, wma_state))]
pub(crate) fn wma_next(new_value: Float, wma_state: PyWmaState) -> PyResult<PyWmaState> {
    let mut wma_state: WmaState = wma_state.into();
    wma_state
        .update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(wma_state.into())
}
