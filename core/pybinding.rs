use pyo3::prelude::pymodule;

use pyo3::prelude::*;

macro_rules! numpy_wrapper {
    (
        $rs_fn:path,
        $py_name:ident,
        $( $arg:ident : $typ:ty $(= $default:expr)? ),* $(,)?
    ) => {
        #[pyfunction(signature = (data, $( $arg $(= $default)? ),* ))]
        fn $py_name<'py>(
            py: pyo3::Python<'py>,
            data: numpy::PyReadonlyArray1<'py, f64>,
            $( $arg : $typ ),*
        ) -> pyo3::PyResult<pyo3::Py<numpy::PyArray1<f64>>> {
            let slice = data.as_slice()?;
            let vec = $rs_fn(slice $(, $arg )*)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?;
            Ok(numpy::PyArray1::from_vec(py, vec).to_owned().into())
        }
    };
}

use crate::indicators::ema::ema as core_ema;
numpy_wrapper!(core_ema, ema,
    window_size: usize,
    smoothing: f64 = 2.0,
);

use crate::indicators::sma::sma as core_sma;
numpy_wrapper!(core_sma, sma,
    window_size: usize,
);

use crate::indicators::rsi::rsi as core_rsi;
numpy_wrapper!(core_rsi, rsi,
    window_size: usize,
);

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    macro_rules! export{
        ($($f:ident),* $(,)?) => {
            $( m.add_function(wrap_pyfunction!($f, m)?)?; )*
        };
    }
    export![ema, sma, rsi];
    Ok(())
}
