use crate::{bird_card::BirdCard, habitat::Habitat};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BirdCardCallback {
    pub card: BirdCard,
    pub habitat: Habitat,
    pub card_idx: usize,
    pub card_player_idx: usize,
}