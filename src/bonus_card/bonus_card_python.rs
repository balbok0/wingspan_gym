use crate::expansion::Expansion;

use super::{BonusCard, ScoringRule};
use pyo3::prelude::*;

#[pymethods]
impl BonusCard {
    #[getter(index)]
    pub fn get_index(&self) -> u16 {
        self.index()
    }

    #[getter(name)]
    pub fn get_name(&self) -> &'static str {
        self.name()
    }

    #[getter(expansion)]
    pub fn get_expansion(&self) -> Expansion {
        self.expansion()
    }

    #[getter(scoring_rule)]
    pub fn get_scoring_rule(&self) -> ScoringRule {
        self.scoring_rule()
    }
}
