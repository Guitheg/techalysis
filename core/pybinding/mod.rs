use pyo3::pymodule;

mod py_ema;
mod py_macd;
mod py_rsi;
mod py_sma;

#[pymodule(gil_used = false)]
mod _core {
    #[pymodule_export]
    use crate::pybinding::py_ema::ema;

    #[pymodule_export]
    use crate::pybinding::py_rsi::rsi;

    #[pymodule_export]
    use crate::pybinding::py_sma::sma;

    #[pymodule_export]
    use crate::pybinding::py_macd::macd;
}
