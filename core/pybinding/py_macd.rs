use crate::indicators::macd as core_macd;
use crate::result::TechnicalysisResult;
use numpy::ndarray::Array2;
use numpy::PyArray2;
use pyo3::pyfunction;

#[pyfunction(signature = (data, fast_period = 12, slow_period = 26, signal_period = 9))]
pub(crate) fn macd<'py>(
    py: pyo3::Python<'py>,
    data: numpy::PyReadonlyArray1<'py, f64>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> pyo3::PyResult<pyo3::Py<numpy::PyArray2<f64>>> {
    let slice = data.as_slice()?;
    let vec = core_macd(slice, fast_period, slow_period, signal_period)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;
    let macd_data = vec.to_vec();
    let array = Array2::from_shape_vec(
        (macd_data.len(), macd_data[0].len()),
        macd_data.into_iter().flatten().collect(),
    )
    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;
    Ok(PyArray2::from_owned_array(py, array).to_owned().into())
}
