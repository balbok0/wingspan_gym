use action::PyAction;
use bird_card::{BirdCard, BirdCardColor};
use bonus_card::{BonusCard, PyScoringRuleType};
use expansion::Expansion;
use food::CostAlternative;
use habitat::Habitat;
use player::Player;
use pyo3::prelude::*;
use step_result::StepResult;
use wingspan_env::PyWingspanEnv;

pub mod bird_card;
pub mod wingspan_env;

mod action;
mod bird_card_callback;
mod bird_feeder;
mod bonus_card;
mod deck_and_holder;
mod end_of_round_goal;
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
    m.add_class::<Player>()?;
    m.add_class::<BirdCard>()?;
    m.add_class::<Habitat>()?;
    m.add_class::<BirdCardColor>()?;
    m.add_class::<CostAlternative>()?;
    m.add_class::<Expansion>()?;
    m.add_class::<BonusCard>()?;
    m.add_class::<PyScoringRuleType>()?;

    Ok(())
}
