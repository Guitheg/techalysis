use pyo3::prelude::*;

mod py_ema;
mod py_macd;
mod py_rsi;
mod py_sma;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_ema::ema, m)?)?;
    m.add_function(wrap_pyfunction!(py_ema::ema_next, m)?)?;
    m.add_class::<py_ema::PyEmaState>()?;

    m.add_function(wrap_pyfunction!(py_rsi::rsi, m)?)?;
    m.add_function(wrap_pyfunction!(py_rsi::rsi_next, m)?)?;
    m.add_class::<py_rsi::PyRsiState>()?;

    m.add_function(wrap_pyfunction!(py_sma::sma, m)?)?;
    m.add_function(wrap_pyfunction!(py_sma::sma_next, m)?)?;
    m.add_class::<py_sma::PySmaState>()?;

    m.add_function(wrap_pyfunction!(py_macd::macd, m)?)?;
    m.add_function(wrap_pyfunction!(py_macd::macd_next, m)?)?;
    m.add_class::<py_macd::PyMacdState>()?;

    Ok(())
}
