use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn wingspan_gym(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let rust_module = PyModule::new(py, "_internal")?;

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add_submodule(&rust_module)?;
    Ok(())
}
