use pyo3::prelude::*;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

/// Binding Python pour la fonction add.
#[pyfunction]
fn py_add(x: i32, y: i32) -> PyResult<i32> {
    Ok(add(x, y))
}

/// Création du module Python.
#[pymodule]
fn technicalysis(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_add, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
