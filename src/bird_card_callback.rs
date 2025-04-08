use crate::{bird_card::BirdCard, habitat::Habitat};
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(get_all)]
pub struct BirdCardCallback {
    pub card: BirdCard,
    pub habitat: Habitat,
    pub card_idx: usize,
    pub card_player_idx: usize,
}

#[pymethods]
impl BirdCardCallback {
    fn __repr__(&self) -> String {
        format!("Callback: ({:?}, {:?}, {}, {})", self.card, self.habitat, self.card_idx, self.card_player_idx)
    }

    fn __str__(&self) -> String {
        format!("Callback: ({:?}, {:?}, {}, {})", self.card, self.habitat, self.card_idx, self.card_player_idx)
    }
}
