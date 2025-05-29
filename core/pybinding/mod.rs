use pyo3::pymodule;

#[macro_export]
macro_rules! numpy_wrapper {
    (
        $rs_fn:path,
        $py_name:ident,
        $( $arg:ident : $typ:ty $(= $default:expr)? ),* $(,)?
    ) => {
        #[pyfunction(signature = (data, $( $arg $(= $default)? ),* ))]
        pub(crate) fn $py_name<'py>(
            py: pyo3::Python<'py>,
            data: numpy::PyReadonlyArray1<'py, f64>,
            $( $arg : $typ ),*
        ) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
            let slice = data.as_slice()?;
            let vec = $rs_fn(slice $(, $arg.into() )*)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;
            Ok(numpy::PyArray1::from_vec(py, vec).to_owned().into())
        }
    };
}

mod py_ema;
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
}
