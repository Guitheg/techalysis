use crate::{indicators::rsi as core_rsi, numpy_wrapper};
use numpy::IntoPyArray;
use pyo3::pyfunction;

numpy_wrapper!(core_rsi, rsi,
    window_size: usize,
);
