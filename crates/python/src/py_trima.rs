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
use techalysis::indicators::trima::{trima_into, TrimaState};
use techalysis::traits::State;
use techalysis::types::Float;

#[pyclass(name = "TrimaState")]
#[derive(Debug, Clone)]
pub struct PyTrimaState {
    #[pyo3(get)]
    pub trima: Float,
    #[pyo3(get)]
    pub weighted_sum: Float,
    #[pyo3(get)]
    pub trailing_sum: Float,
    #[pyo3(get)]
    pub heading_sum: Float,
    #[pyo3(get)]
    pub last_window: Vec<Float>,
    #[pyo3(get)]
    pub inv_weight_sum: Float,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyTrimaState {
    #[new]
    pub fn new(
        trima: Float,
        weighted_sum: Float,
        trailing_sum: Float,
        heading_sum: Float,
        last_window: Vec<Float>,
        inv_weight_sum: Float,
        period: usize,
    ) -> Self {
        PyTrimaState {
            trima,
            weighted_sum,
            trailing_sum,
            heading_sum,
            last_window,
            inv_weight_sum,
            period,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!("TrimaState(trima={}, period={})", self.trima, self.period)
    }
}
impl From<TrimaState> for PyTrimaState {
    fn from(state: TrimaState) -> Self {
        PyTrimaState {
            trima: state.trima,
            weighted_sum: state.weighted_sum,
            trailing_sum: state.trailing_sum,
            heading_sum: state.heading_sum,
            last_window: state.last_window.into(),
            inv_weight_sum: state.inv_weight_sum,
            period: state.period,
        }
    }
}

impl From<PyTrimaState> for TrimaState {
    fn from(py_state: PyTrimaState) -> Self {
        TrimaState {
            trima: py_state.trima,
            weighted_sum: py_state.weighted_sum,
            trailing_sum: py_state.trailing_sum,
            heading_sum: py_state.heading_sum,
            last_window: py_state.last_window.into(),
            inv_weight_sum: py_state.inv_weight_sum,
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn trima(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyTrimaState)> {
    let len = data.len();
    let slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let state = py
            .allow_threads(|| trima_into(slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [slice.len()], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = trima_into(slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, trima_state))]
pub(crate) fn trima_next(new_value: Float, trima_state: PyTrimaState) -> PyResult<PyTrimaState> {
    let mut trima_state: TrimaState = trima_state.into();
    trima_state
        .update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(trima_state.into())
}
