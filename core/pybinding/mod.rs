pub mod sma;
use pyo3::prelude::pymodule;

#[pymodule]
mod core {
    #[pymodule_export]
    use crate::pybinding::sma::sma;
}
