use rand::{rngs::StdRng, Rng};

use crate::{action::Action, error::{WingError, WingResult}, food::FoodIndex};

fn sample_dice(rng: &mut StdRng, num_times: usize) -> Vec<u8> {
    (0..num_times).map(|_| rng.gen_range(0..6u8)).collect()
}

#[derive(Debug, Clone)]
pub struct BirdFeeder {
    dice_in_birdfeeder: Vec<u8>,
    dice_out_birdfeeder: Vec<u8>,
}

impl Default for BirdFeeder {
    fn default() -> Self {
        Self { dice_in_birdfeeder: Default::default(), dice_out_birdfeeder: Default::default() }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum BirdFeederActionResult {
    GainFood(FoodIndex),
    FollowupAction(Action),
}

impl BirdFeeder {
    pub fn reroll(&mut self, rng: &mut StdRng) {
        self.dice_out_birdfeeder.clear();
        self.dice_in_birdfeeder = sample_dice(rng, 5);
    }

    pub fn take_dice_and_update_state(&mut self, rng: &mut StdRng, idx: u8) -> WingResult<BirdFeederActionResult> {
        let idx = idx as usize;

        if idx == self.dice_in_birdfeeder.len() {
            // Action equal to size of bird-feeder is a re-roll
            if self.can_reroll() {
                // Reroll is valid. Do it
                self.reroll(rng);

                // Player still should get food after re-roll
                return Ok(BirdFeederActionResult::FollowupAction(Action::GetFood));
            } else {
                // Reroll is not valid. This is not a performable action
                return Err(WingError::InvalidAction);
            }
        } else if idx > self.dice_in_birdfeeder.len() {
            return Err(WingError::InvalidAction);
        }

        // Update dice in bird feeder
        let dice_face = self.dice_in_birdfeeder.remove(idx);
        self.dice_out_birdfeeder.push(dice_face);

        // Update state of env
        let result = match dice_face {
            0 | 1 | 2 | 3 | 4 => BirdFeederActionResult::GainFood(FoodIndex::from(dice_face)),
            5 => BirdFeederActionResult::FollowupAction(Action::GetFoodChoice(Box::new([FoodIndex::Seed, FoodIndex::Invertebrate]))),
            _ => panic!("Incorrect dice face: {}", dice_face),
        };

        Ok(result)
    }

    pub fn count(&self, food_idx: FoodIndex) -> usize {
        let allowed_dice = match food_idx {
            FoodIndex::Fish | FoodIndex::Fruit | FoodIndex::Rodent => vec![food_idx as u8],
            FoodIndex::Invertebrate | FoodIndex::Seed => vec![food_idx as u8, 5],
        };

        self.dice_in_birdfeeder.iter()
            .filter(|d| allowed_dice.contains(*d))
            .count()
    }

    pub fn take_specific_food(&mut self, food_idx: FoodIndex) -> WingResult<()> {
        let allowed_dice = match food_idx {
            FoodIndex::Fish | FoodIndex::Fruit | FoodIndex::Rodent => vec![food_idx as u8],
            FoodIndex::Invertebrate | FoodIndex::Seed => vec![food_idx as u8, 5],
        };

        let dice_to_remove = self.dice_in_birdfeeder.iter().enumerate()
            .filter_map(|(idx, d)|
                if allowed_dice.contains(d) {
                    Some(idx)
                } else {
                    None
                }
            )
            .next();

        let dice_to_remove = dice_to_remove.ok_or(WingError::InvalidAction)?;

        self.dice_in_birdfeeder.remove(dice_to_remove);
        Ok(())
    }

    pub fn num_dice_in(&self) -> usize {
        self.dice_in_birdfeeder.len()
    }

    pub fn num_dice_out(&self) -> usize {
        self.dice_out_birdfeeder.len()
    }

    pub fn num_actions(&self) -> usize {
        if self.can_reroll() {
            self.num_dice_in()
        } else {
            self.num_dice_in() + 1
        }
    }

    pub fn can_reroll(&self) -> bool {
        self.dice_in_birdfeeder.iter().min() == self.dice_in_birdfeeder.iter().max()
    }
}
