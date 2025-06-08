use pyo3::prelude::*;

mod py_dema;
mod py_wma;
mod py_bbands;
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

    m.add_function(wrap_pyfunction!(py_bbands::bbands, m)?)?;
    m.add_function(wrap_pyfunction!(py_bbands::bbands_next, m)?)?;
    m.add_class::<py_bbands::PyBBandsState>()?;
    m.add_class::<py_bbands::PyBBandsMA>()?;

    m.add_function(wrap_pyfunction!(py_wma::wma, m)?)?;
    m.add_function(wrap_pyfunction!(py_wma::wma_next, m)?)?;
    m.add_class::<py_wma::PyWmaState>()?;

    m.add_function(wrap_pyfunction!(py_dema::dema, m)?)?;
    m.add_function(wrap_pyfunction!(py_dema::dema_next, m)?)?;
    m.add_class::<py_dema::PyDemaState>()?;

    Ok(())
}
