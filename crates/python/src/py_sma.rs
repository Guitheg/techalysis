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
use techalysis::indicators::sma::{sma_into, SmaState};
use techalysis::traits::State;
use techalysis::types::Float;

#[pyclass(name = "SmaState")]
#[derive(Debug, Clone)]
pub struct PySmaState {
    #[pyo3(get)]
    pub sma: Float,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub window: Vec<Float>,
}
#[pymethods]
impl PySmaState {
    #[new]
    pub fn new(sma: Float, period: usize, window: Vec<Float>) -> Self {
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
            window: state.last_window.into(),
        }
    }
}

impl From<PySmaState> for SmaState {
    fn from(py_state: PySmaState) -> Self {
        SmaState {
            sma: py_state.sma,
            period: py_state.period,
            last_window: py_state.window.into(),
        }
    }
}

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn sma(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PySmaState)> {
    let len = data.len();
    let slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let state = py
            .allow_threads(|| sma_into(slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [slice.len()], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = sma_into(slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, sma_state))]
pub(crate) fn sma_next(new_value: Float, sma_state: PySmaState) -> PyResult<PySmaState> {
    let mut sma_state: SmaState = sma_state.into();
    sma_state
        .update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(sma_state.into())
}
