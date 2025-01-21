use crate::{bird_card::BirdCard, resource::Resources};
use pyo3::prelude::*;


#[derive(Debug, Clone)]
#[pyclass]
pub struct Player {
    #[pyo3(get)]
    resources: Resources,
    bird_cards: Vec<BirdCard>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            resources: [1, 1, 1, 1, 1],
            bird_cards: vec![]
        }
    }
}

impl Player {
    pub fn new(bird_cards: Vec<BirdCard>) -> Self {
        Self {
            resources: [1, 1, 1, 1, 1],
            bird_cards,
        }
    }
}

#[pymethods]
impl Player {
    #[getter]
    pub fn bird_cards(&self) -> Vec<u16> {
        self.bird_cards.iter().map(BirdCard::index).collect()
    }
}