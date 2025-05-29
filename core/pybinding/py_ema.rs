use crate::{indicators::ema as core_ema, numpy_wrapper};
use pyo3::pyfunction;

numpy_wrapper!(core_ema, ema,
    window_size: usize,
    alpha: Option<f64> = None,
);
