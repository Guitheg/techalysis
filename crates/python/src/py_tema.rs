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
use techalysis::indicators::ema::period_to_alpha;
use techalysis::indicators::tema::{tema_into, TemaState};
use techalysis::traits::State;
use techalysis::types::Float;

#[pyclass(name = "TemaState")]
#[derive(Debug, Clone)]
pub struct PyTemaState {
    #[pyo3(get)]
    pub tema: Float,
    #[pyo3(get)]
    pub ema_1: Float,
    #[pyo3(get)]
    pub ema_2: Float,
    #[pyo3(get)]
    pub ema_3: Float,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub alpha: Option<Float>,
}
#[pymethods]
impl PyTemaState {
    #[new]
    pub fn new(
        tema: Float,
        ema_1: Float,
        ema_2: Float,
        ema_3: Float,
        period: usize,
        alpha: Option<Float>,
    ) -> Self {
        PyTemaState {
            tema,
            ema_1,
            ema_2,
            ema_3,
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
            "TemaState(tema={}, ema_1={}, ema_2={}, ema_3={}, period={}, alpha={:?})",
            self.tema, self.ema_1, self.ema_2, self.ema_3, self.period, self.alpha
        )
    }
}
impl From<TemaState> for PyTemaState {
    fn from(state: TemaState) -> Self {
        PyTemaState {
            tema: state.tema,
            ema_1: state.ema_1,
            ema_2: state.ema_2,
            ema_3: state.ema_3,
            period: state.period,
            alpha: state.alpha.into(),
        }
    }
}

impl From<PyTemaState> for TemaState {
    fn from(py_state: PyTemaState) -> Self {
        TemaState {
            tema: py_state.tema,
            ema_1: py_state.ema_1,
            ema_2: py_state.ema_2,
            ema_3: py_state.ema_3,
            period: py_state.period,
            alpha: py_state.alpha.unwrap_or(
                period_to_alpha(py_state.period, None).unwrap_or(2.0 / py_state.period as Float),
            ),
        }
    }
}

#[pyfunction(signature = (data, period = 14, alpha = None, release_gil = false))]
pub(crate) fn tema(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    alpha: Option<Float>,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyTemaState)> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| tema_into(input_slice, period, alpha, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = tema_into(input_slice, period, alpha, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, tema_state))]
pub(crate) fn tema_next(new_value: Float, tema_state: PyTemaState) -> PyResult<PyTemaState> {
    let mut tema_state: TemaState = tema_state.into();
    tema_state
        .update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(tema_state.into())
}
