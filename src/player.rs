use crate::{action::Action, bird_card::{is_enough_food_to_play_a_card, BirdCard}, error::{WingError, WingResult}, food::Foods, habitat::Habitat, player_mat::PlayerMat};
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
    pub(crate) _playable_card_hab_combos: Vec<(BirdCard, Habitat, usize)>
}

impl Default for Player {
    fn default() -> Self {
        Self {
            foods: [1, 1, 1, 1, 1],
            bird_cards: vec![],
            turns_left: 8,
            mat: Default::default(),
            _playable_card_hab_combos: vec![],
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
            _playable_card_hab_combos: vec![],
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
        for (idx, card) in self.bird_cards.iter().enumerate() {
            if is_enough_food_to_play_a_card(&card, &self.foods) {
                let mut cur_card_habitat_combos: Vec<_> = self.mat
                    .playable_habitats(&card)
                    .into_iter()
                    .map(|habitat| (*card, habitat, idx))
                    .collect();
                playable_cards.append(&mut cur_card_habitat_combos)
            }
        }
        self._playable_card_hab_combos = playable_cards;

        !self._playable_card_hab_combos.is_empty()
    }

    pub fn play_a_bird_card(&mut self, bird_card_idx: u8) -> WingResult<Vec<Action>> {
        let bird_card_idx = bird_card_idx as usize;
        if bird_card_idx >= self._playable_card_hab_combos.len() {
            return Err(WingError::InvalidAction);
        }

        let (bird_card, hab, orig_card_idx) = self._playable_card_hab_combos[bird_card_idx];

        let result = self.pay_bird_cost(&bird_card)?;
        self.mat.put_bird_card(bird_card, &hab)?;
        self.bird_cards.remove(orig_card_idx);

        Ok(result)
    }

    fn pay_bird_cost(&mut self, bird_card: &BirdCard) -> WingResult<Vec<Action>> {
        let (costs, total, is_alt) = bird_card.cost();

        if !is_enough_food_to_play_a_card(bird_card, &self.foods) {
            return Err(WingError::InvalidAction);
        }

        let result = match is_alt {
            crate::food::CostAlternative::Yes => {
                // Note: No need to keep track of total cost since it does not appear in "/" (or CostAlternative::Yes) cards

                // First determine what are the discard options
                let mut discard_options = vec![];
                for (food_idx, food_cost) in costs.iter().enumerate() {
                    if let Some(food_cost) = food_cost {
                        if self.foods[food_idx] >= *food_cost {
                            discard_options.push((food_idx as usize, *food_cost));
                        }
                    }
                }

                // If there is only one option, just do it
                if discard_options.len() == 1 {
                    let (food_idx, food_cost) = discard_options.pop().unwrap();

                    self.foods[food_idx] -= food_cost;
                    vec![]
                } else {
                    vec![Action::DiscardFoodChoice(discard_options.into_boxed_slice())]
                }
            },
            crate::food::CostAlternative::No => {
                // No Cost Alternative, so no choices needed
                let mut total_defined_cost = 0;
                for (food_idx, food_cost) in costs.iter().enumerate() {
                    if let Some(food_cost) = food_cost {
                        self.foods[food_idx] -= *food_cost;
                        total_defined_cost += *food_cost;
                    }
                }

                // For all of the arbitrary costs, return actions needed
                (0..total - total_defined_cost).map(|_| Action::DiscardFood).collect()
            }
        };

        Ok(result)
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

    pub fn birds_on_mat(&self) -> [Vec<u16>; 3] {
        [
            self.mat.get_row(&Habitat::Forest).birds.iter().map(BirdCard::index).collect(),
            self.mat.get_row(&Habitat::Grassland).birds.iter().map(BirdCard::index).collect(),
            self.mat.get_row(&Habitat::Wetland).birds.iter().map(BirdCard::index).collect(),
        ]
    }
}