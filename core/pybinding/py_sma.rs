use crate::{indicators::sma as core_sma, numpy_wrapper};
use numpy::IntoPyArray;
use pyo3::pyfunction;

numpy_wrapper!(core_sma, sma,
    window_size: usize,
);
