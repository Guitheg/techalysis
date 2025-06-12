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
use techalib::indicators::kama::{kama_into, KamaState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "KamaState")]
#[derive(Debug, Clone)]
pub struct PyKamaState {
    #[pyo3(get)]
    pub kama: Float,
    #[pyo3(get)]
    pub roc_sum: Float,
    #[pyo3(get)]
    pub last_window: Vec<Float>,
    #[pyo3(get)]
    pub trailing_value: Float,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyKamaState {
    #[new]
    pub fn new(
        kama: Float,
        roc_sum: Float,
        last_window: Vec<Float>,
        trailing_value: Float,
        period: usize,
    ) -> Self {
        PyKamaState {
            kama,
            roc_sum,
            last_window,
            trailing_value,
            period,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<KamaState> for PyKamaState {
    fn from(state: KamaState) -> Self {
        PyKamaState {
            kama: state.kama,
            roc_sum: state.roc_sum,
            last_window: state.last_window.into(),
            trailing_value: state.trailing_value,
            period: state.period,
        }
    }
}

impl From<PyKamaState> for KamaState {
    fn from(py_state: PyKamaState) -> Self {
        KamaState {
            kama: py_state.kama,
            roc_sum: py_state.roc_sum,
            last_window: py_state.last_window.into(),
            trailing_value: py_state.trailing_value,
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (data, period = 30, release_gil = false))]
pub(crate) fn kama(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyKamaState)> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| kama_into(input_slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = kama_into(input_slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, kama_state))]
pub(crate) fn kama_next(new_value: Float, kama_state: PyKamaState) -> PyResult<PyKamaState> {
    let mut kama_state: KamaState = kama_state.into();
    kama_state
        .update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(kama_state.into())
}
