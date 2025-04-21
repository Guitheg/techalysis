use pyo3::prelude::pymodule;

#[pymodule(gil_used = false)]
mod core {
    use numpy::{PyArray1, PyReadonlyArray1};
    use pyo3::{exceptions::PyValueError, prelude::*};

    use crate::features::ema::ema as core_ema;
    #[pyfunction(signature = (data, window_size, smoothing = 2.0))]
    pub fn ema(
        py: Python,
        data: PyReadonlyArray1<f64>,
        window_size: usize,
        smoothing: f64,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let data_slice = data.as_slice()?;
        let result = core_ema(data_slice, window_size, smoothing)
            .map_err(|e| PyValueError::new_err(format!("Error computing EMA: {:?}", e)))?;
        let result_array = PyArray1::from_vec(py, result).to_owned();
        Ok(result_array.into())
    }

    use crate::features::sma::sma as core_sma;
    #[pyfunction(signature = (data, window_size, handle_nan = false))]
    pub fn sma(
        py: Python,
        data: PyReadonlyArray1<f64>,
        window_size: usize,
        handle_nan: bool,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let data_slice = data.as_slice()?;
        let result = core_sma(data_slice, window_size, handle_nan)
            .map_err(|e| PyValueError::new_err(format!("Error computing SMA: {:?}", e)))?;
        let result_array = PyArray1::from_vec(py, result).to_owned();
        Ok(result_array.into())
    }
}
