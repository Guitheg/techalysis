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
use techalysis::indicators::macd::{macd_into, MacdState};
use techalysis::traits::State;
use techalysis::types::Float;

#[pyclass(name = "MacdState")]
#[derive(Debug, Clone)]
pub struct PyMacdState {
    #[pyo3(get)]
    pub macd: Float,
    #[pyo3(get)]
    pub signal: Float,
    #[pyo3(get)]
    pub histogram: Float,
    #[pyo3(get)]
    pub fast_ema: Float,
    #[pyo3(get)]
    pub slow_ema: Float,
    #[pyo3(get)]
    pub fast_period: usize,
    #[pyo3(get)]
    pub slow_period: usize,
    #[pyo3(get)]
    pub signal_period: usize,
}
#[pymethods]
impl PyMacdState {
    #[new]
    pub fn new(
        macd: Float,
        signal: Float,
        histogram: Float,
        fast_ema: Float,
        slow_ema: Float,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Self {
        PyMacdState {
            macd,
            signal,
            histogram,
            fast_ema,
            slow_ema,
            fast_period,
            slow_period,
            signal_period,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!(
            "MacdState(macd={}, signal={}, histogram={}, fast_ema={}, slow_ema={}, fast_period={}, slow_period={}, signal_period={})",
            self.macd, self.signal, self.histogram, self.fast_ema, self.slow_ema, self.fast_period, self.slow_period, self.signal_period
        )
    }
}
impl From<MacdState> for PyMacdState {
    fn from(state: MacdState) -> Self {
        PyMacdState {
            macd: state.macd,
            signal: state.signal,
            histogram: state.histogram,
            fast_ema: state.fast_ema,
            slow_ema: state.slow_ema,
            fast_period: state.fast_period,
            slow_period: state.slow_period,
            signal_period: state.signal_period,
        }
    }
}
impl From<PyMacdState> for MacdState {
    fn from(py_state: PyMacdState) -> Self {
        MacdState {
            macd: py_state.macd,
            signal: py_state.signal,
            histogram: py_state.histogram,
            fast_ema: py_state.fast_ema,
            slow_ema: py_state.slow_ema,
            fast_period: py_state.fast_period,
            slow_period: py_state.slow_period,
            signal_period: py_state.signal_period,
        }
    }
}

#[pyfunction(signature = (data, fast_period = 12, slow_period = 26, signal_period = 9, release_gil = false))]
pub(crate) fn macd(
    py: Python,
    data: PyReadonlyArray1<Float>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    release_gil: bool,
) -> PyResult<(
    Py<PyArray1<Float>>,
    Py<PyArray1<Float>>,
    Py<PyArray1<Float>>,
    PyMacdState,
)> {
    let len = data.len();
    let input_data = data.as_slice()?;

    if release_gil {
        let mut output_macd = vec![0.0; len];
        let mut output_signal = vec![0.0; len];
        let mut output_histogram = vec![0.0; len];

        let macd_state = py
            .allow_threads(|| {
                macd_into(
                    input_data,
                    fast_period,
                    slow_period,
                    signal_period,
                    output_macd.as_mut_slice(),
                    output_signal.as_mut_slice(),
                    output_histogram.as_mut_slice(),
                )
            })
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((
            output_macd.into_pyarray(py).into(),
            output_signal.into_pyarray(py).into(),
            output_histogram.into_pyarray(py).into(),
            macd_state.into(),
        ));
    } else {
        let py_array_macd = PyArray1::<Float>::zeros(py, [len], false);
        let output_macd_data = unsafe { py_array_macd.as_slice_mut()? };

        let py_array_signal = PyArray1::<Float>::zeros(py, [len], false);
        let output_signal_data = unsafe { py_array_signal.as_slice_mut()? };

        let py_array_histogram = PyArray1::<Float>::zeros(py, [len], false);
        let output_histogram_data = unsafe { py_array_histogram.as_slice_mut()? };

        let macd_state = macd_into(
            input_data,
            fast_period,
            slow_period,
            signal_period,
            output_macd_data,
            output_signal_data,
            output_histogram_data,
        )
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        return Ok((
            py_array_macd.into(),
            py_array_signal.into(),
            py_array_histogram.into(),
            macd_state.into(),
        ));
    }
}

#[pyfunction(signature = (new_value, macd_state,))]
pub(crate) fn macd_next(new_value: Float, macd_state: PyMacdState) -> PyResult<PyMacdState> {
    let mut state: MacdState = macd_state.into();
    state.update(new_value).map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
    Ok(state.into())
}
