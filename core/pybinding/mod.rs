use pyo3::prelude::pymodule;

#[pymodule]
mod core {
    use crate::features::sma::sma as core_sma;
    use pyo3::{exceptions::PyValueError, pyfunction, PyResult};

    #[pyfunction]
    pub fn sma(
        start_idx: usize,
        end_idx: usize,
        data: Vec<f64>,
        period: usize,
    ) -> PyResult<(usize, Vec<f64>)> {
        core_sma(start_idx, end_idx, &data, period)
            .map_err(|e| PyValueError::new_err(format!("Error computing SMA: {:?}", e)))
    }
}
