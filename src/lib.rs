use pyo3::prelude::*;
use wingspan_env::PyWingspanEnv;

pub mod bird_card;
pub mod wingspan_env;
mod expansion;
mod resource;
mod habitat;
mod nest;
mod player;
mod action;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn wingspan_gym(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWingspanEnv>()?;
    Ok(())
}
