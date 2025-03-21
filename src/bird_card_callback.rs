use crate::{bird_card::BirdCard, habitat::Habitat};
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass]
pub struct BirdCardCallback {
    pub card: BirdCard,
    pub habitat: Habitat,
    pub card_idx: usize,
    pub card_player_idx: usize,
}
