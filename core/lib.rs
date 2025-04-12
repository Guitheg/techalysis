use pyo3::prelude::*;

/// Binding Python pour la fonction add.
#[pyfunction]
fn add(x: i32, y: i32) -> PyResult<i32> {
    Ok(x + y)
}

/// Création du module Python.
#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}

#[pymodule]
fn technicalysis(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(m)
}
