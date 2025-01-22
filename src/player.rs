use crate::{bird_card::{is_enough_food_to_play_a_card, BirdCard}, error::{WingError, WingResult}, food::Foods, player_mat::PlayerMat};
use pyo3::prelude::*;


#[derive(Debug, Clone)]
#[pyclass]
pub struct Player {
    #[pyo3(get)]
    pub(crate) foods: Foods,
    pub(crate) bird_cards: Vec<BirdCard>,
    #[pyo3(get)]
    pub(crate) turns_left: u8,

    pub(crate) mat: PlayerMat,

    // Optimization that uses a fact, that before every bird play we check for resources etc.
    pub(crate) _playable_bird_cards: Vec<BirdCard>
}

impl Default for Player {
    fn default() -> Self {
        Self {
            foods: [1, 1, 1, 1, 1],
            bird_cards: vec![],
            turns_left: 8,
            mat: Default::default(),
            _playable_bird_cards: vec![],
        }
    }
}

impl Player {
    pub fn new(bird_cards: Vec<BirdCard>) -> Self {
        Self {
            foods: [1, 1, 1, 1, 1],
            bird_cards,
            turns_left: 8,
            mat: Default::default(),
            _playable_bird_cards: vec![],
        }
    }

    pub fn set_turns_left(&mut self, turns_left: u8) {
        self.turns_left = turns_left;
    }

    pub fn discard_bird_card(&mut self, index: usize) -> WingResult<()> {
        if index >= self.bird_cards.len() {
            return Err(WingError::InvalidAction);
        }

        self.bird_cards.remove(index);
        Ok(())
    }

    pub fn discard_food(&mut self, index: usize) -> WingResult<()> {
        if index >= self.foods.len() {
            return Err(WingError::InvalidAction);
        }
        if self.foods[index] == 0 {
            return Err(WingError::InvalidAction);
        }

        self.foods[index] -= 1;
        Ok(())
    }

    pub fn discard_food_or_bird_card(&mut self, index: usize) -> WingResult<()> {
        if index < 5 {
            self.discard_food(index)
        } else {
            self.discard_bird_card(index - 5)
        }
    }

    pub fn can_play_a_bird_card(&mut self) -> bool {
        let mut playable_cards = vec![];
        for card in &self.bird_cards {
            if self.mat.can_be_played(&card) && is_enough_food_to_play_a_card(&card, &self.foods) {
                playable_cards.push(*card);
            }
        }
        self._playable_bird_cards = playable_cards;

        !self._playable_bird_cards.is_empty()
    }

    pub fn can_discard_food(&self) -> bool {
        self.foods.iter().sum::<u8>() > 0
    }

    pub fn can_discard_bird_card(&self) -> bool {
        self.bird_cards.len() > 0
    }
}

#[pymethods]
impl Player {
    #[getter]
    pub fn bird_cards(&self) -> Vec<u16> {
        self.bird_cards.iter().map(BirdCard::index).collect()
    }
}