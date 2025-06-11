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
use pyo3::{pymethods, exceptions::PyValueError, pyclass, pyfunction, Py, PyResult, Python};
use techalysis::indicators::t3::{t3_into, T3Coefficients, T3EmaValues, T3State};
use techalysis::traits::State;
use techalysis::types::Float;

#[pyclass(name = "T3State")]
#[derive(Debug, Clone)]
pub struct PyT3State {
    #[pyo3(get)]
    pub t3: Float,

    #[pyo3(get)]
    pub ema1: Float,
    #[pyo3(get)]
    pub ema2: Float,
    #[pyo3(get)]
    pub ema3: Float,
    #[pyo3(get)]
    pub ema4: Float,
    #[pyo3(get)]
    pub ema5: Float,
    #[pyo3(get)]
    pub ema6: Float,

    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub alpha: Float,
    #[pyo3(get)]
    pub vfactor: Float,

    #[pyo3(get)]
    pub c1: Float,
    #[pyo3(get)]
    pub c2: Float,
    #[pyo3(get)]
    pub c3: Float,
    #[pyo3(get)]
    pub c4: Float,
}
#[pymethods]
impl PyT3State {
    #[new]
    pub fn new(
        t3: Float,
        ema1: Float,
        ema2: Float,
        ema3: Float,
        ema4: Float,
        ema5: Float,
        ema6: Float,
        period: usize,
        alpha: Float,
        vfactor: Float,
        c1: Float,
        c2: Float,
        c3: Float,
        c4: Float,
    ) -> Self {
        PyT3State {
            t3,
            ema1,
            ema2,
            ema3,
            ema4,
            ema5,
            ema6,
            period,
            alpha,
            vfactor,
            c1,
            c2,
            c3,
            c4,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "T3State(t3={}, ema1={}, ema2={}, ema3={}, ema4={}, ema5={}, ema6={}, period={}, alpha={}, vfactor={}, c1={}, c2={}, c3={}, c4={})",
            self.t3, self.ema1, self.ema2, self.ema3, self.ema4, self.ema5, self.ema6,
            self.period, self.alpha, self.vfactor, self.c1, self.c2, self.c3, self.c4
        )
    }
}
impl From<T3State> for PyT3State {
    fn from(state: T3State) -> Self {
        PyT3State {
            t3: state.t3,
            ema1: state.ema_values.ema1,
            ema2: state.ema_values.ema2,
            ema3: state.ema_values.ema3,
            ema4: state.ema_values.ema4,
            ema5: state.ema_values.ema5,
            ema6: state.ema_values.ema6,
            period: state.period,
            alpha: state.alpha,
            vfactor: state.volume_factor,
            c1: state.t3_coefficients.c1,
            c2: state.t3_coefficients.c2,
            c3: state.t3_coefficients.c3,
            c4: state.t3_coefficients.c4,
        }
    }
}

impl From<PyT3State> for T3State {
    fn from(py_state: PyT3State) -> Self {
        T3State {
            t3: py_state.t3,
            ema_values: T3EmaValues {
                ema1: py_state.ema1,
                ema2: py_state.ema2,
                ema3: py_state.ema3,
                ema4: py_state.ema4,
                ema5: py_state.ema5,
                ema6: py_state.ema6,
            },
            period: py_state.period,
            alpha: py_state.alpha,
            volume_factor: py_state.vfactor,
            t3_coefficients: T3Coefficients {
                c1: py_state.c1,
                c2: py_state.c2,
                c3: py_state.c3,
                c4: py_state.c4,
            },
        }
    }
}

#[pyfunction(signature = (data, period = 5, vfactor = 0.7, alpha = None, release_gil = false))]
pub(crate) fn t3(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    vfactor: Float,
    alpha: Option<Float>,
    release_gil: bool,
) -> PyResult<(
    Py<PyArray1<Float>>,
    PyT3State
)> {
    let len = data.len();
    let input_slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        
        let state = py
            .allow_threads(|| t3_into(input_slice, period, vfactor, alpha, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((
            output.into_pyarray(py).into(),
            state.into()
        ))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = t3_into(input_slice, period, vfactor, alpha, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((
            py_array_out.into(),
            state.into()
        ))
    }
}

#[pyfunction(signature = (new_value, t3_state))]
pub(crate) fn t3_next(
    new_value: Float,
    t3_state: PyT3State
) -> PyResult<PyT3State> {
    let mut t3_state: T3State = t3_state.into();
    t3_state.update(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(t3_state.into())
}
