use pyo3::prelude::*;
use wingspan_env::PyWingspanEnv;

pub mod bird_card;
pub mod wingspan_env;
mod bird_feeder;
mod deck_and_holder;
mod expansion;
mod food;
mod habitat;
mod nest;
mod error;
mod player;
mod player_mat;
mod action;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn wingspan_gym(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWingspanEnv>()?;
    Ok(())
}
