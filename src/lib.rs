use action::PyAction;
use pyo3::prelude::*;
use step_result::StepResult;
use wingspan_env::PyWingspanEnv;

pub mod bird_card;
pub mod wingspan_env;

mod action;
mod bird_feeder;
mod bonus_card;
mod deck_and_holder;
mod error;
mod expansion;
mod food;
mod habitat;
mod nest;
mod player;
mod player_mat;
mod step_result;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn wingspan_gym(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWingspanEnv>()?;
    m.add_class::<StepResult>()?;
    m.add_class::<PyAction>()?;

    Ok(())
}
