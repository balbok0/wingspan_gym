use crate::{error::{WingError, WingResult}, habitat::Habitat, wingspan_env::WingspanEnv};
use pyo3::prelude::*;


#[derive(Debug, Clone)]
pub enum Action {
    // First decision of the turn (i.e. play a bird, forest, grassland, wetland)
    ChooseAction,

    PlayBird,

    // Get resource actions
    GetFood,
    GetFoodChoice(Box<[usize]>),
    GetEgg,
    GetBirdCard,

    // Discard Food Or Bird Card
    DiscardFoodOrBirdCard,
    // Discard Bird Card
    DiscardBirdCard,
    // Discard Food
    DiscardFood,
    DiscardFoodChoice(Box<[(usize, u8)]>), // Discard food of choice N times
    // Discard Egg
    DiscardEgg,

    // Do something (typically discard to perform an action)
    DoThen(Box<Action>, Box<Action>),
}

impl Action {
    pub fn perform_action(&self, action_idx: u8, env: &mut WingspanEnv) -> WingResult<()> {
        match self {
            Action::ChooseAction => {
                let habitat = match action_idx {
                    1 => Habitat::Forest,
                    2 => Habitat::Grassland,
                    3 => Habitat::Wetland,
                    0 => {
                        // Play a card action
                        // Check if a bird card can be played
                        if !Action::PlayBird.is_performable(env) {
                            return Err(WingError::InvalidAction)
                        }

                        env._action_queue.push(Action::PlayBird);
                        return Ok(());
                    },
                    _ => {
                        return Err(WingError::InvalidAction)
                    }
                };

                env.populate_action_queue_from_habitat_action(&habitat);

                Ok(())
            },
            Action::PlayBird => {
                let mut followup_actions = env.current_player_mut().play_a_bird_card(action_idx)?;

                env._action_queue.append(&mut followup_actions);
                Ok(())
            },
            Action::GetFood => {
                match env._bird_feeder.take_dice_and_update_state(&mut env.rng, action_idx)? {
                    crate::bird_feeder::BirdFeederActionResult::GainFood(food_idx) => env.current_player_mut().foods[food_idx] += 1,
                    crate::bird_feeder::BirdFeederActionResult::FollowupAction(action) => env._action_queue.push(action),
                }
                Ok(())
            },
            Action::GetFoodChoice(choices) => {
                let action_idx = action_idx as usize;
                if action_idx >= choices.len() {
                    Err(WingError::InvalidAction)
                } else {
                    env.current_player_mut().foods[choices[action_idx]] += 1;
                    Ok(())
                }
            },
            Action::GetEgg => env.current_player_mut().mat.place_egg(action_idx),
            Action::GetBirdCard => {
                let card = env._bird_deck.draw_card(action_idx)?;
                env.current_player_mut().bird_cards.push(card);
                Ok(())
            },
            Action::DiscardFoodOrBirdCard => {
                env.current_player_mut().discard_food_or_bird_card(action_idx as usize)
            },
            Action::DiscardBirdCard => {
                env.current_player_mut().discard_bird_card(action_idx as usize)
            },
            Action::DiscardFood => {
                env.current_player_mut().discard_food(action_idx as usize, 1)
            },
            Action::DiscardFoodChoice(choices) => {
                let (food_idx, num_food) = choices.get(action_idx as usize).ok_or(WingError::InvalidAction)?;

                env.current_player_mut().discard_food(*food_idx, *num_food)
            },
            Action::DiscardEgg => env.current_player_mut().mat.discard_egg(action_idx),
            Action::DoThen(a, b) => {
                match action_idx {
                    0 => {
                        // Option accepted
                        env._action_queue.push(*b.clone());
                        env._action_queue.push(*a.clone());
                        Ok(())
                    },
                    1 => {
                        // Option rejected
                        Ok(())
                    },
                    _ => Err(WingError::InvalidAction)
                }
            }
            x => {
                println!("Action not implemented: {:?}", x);
                todo!()
            },
        }
    }

    pub fn is_performable(&self, env: &mut WingspanEnv) -> bool {
        match self {
            Action::ChooseAction => true,
            Action::PlayBird => env.current_player_mut().can_play_a_bird_card(),
            Action::GetFood => true,
            Action::GetFoodChoice(_) => true,
            Action::GetEgg => env.current_player().mat.can_place_egg(),
            Action::GetBirdCard => env.current_player().bird_cards.len() < env.config().hand_limit.into(),
            Action::DiscardFoodOrBirdCard => Action::DiscardFood.is_performable(env) || Action::DiscardBirdCard.is_performable(env),
            Action::DiscardBirdCard => env.current_player().can_discard_bird_card(),
            Action::DiscardFood => env.current_player().can_discard_food(),
            Action::DiscardFoodChoice(choices) => {
                let foods = &env.current_player().foods;
                choices
                    .iter()
                    .map(|(idx, cost)| foods[*idx] >= *cost)
                    .reduce(|a, b| a || b)
                    .unwrap_or(true)
            },
            Action::DiscardEgg => env.current_player().mat.can_discard_egg(),
            Action::DoThen(action_req, action_reward) => action_req.is_performable(env) && action_reward.is_performable(env),
        }
    }

    pub fn action_space_size(&self, env: &WingspanEnv) -> usize {
        match self {
            Action::ChooseAction => 4,
            Action::PlayBird => {
                env.current_player()._playable_card_hab_combos.len()
            },
            Action::GetFood => env._bird_feeder.num_actions(),
            Action::GetFoodChoice(choices) => choices.len(),
            Action::GetEgg => env.current_player().mat.num_spots_to_place_eggs(),
            Action::GetBirdCard => env._bird_deck.num_actions(),
            Action::DiscardFoodOrBirdCard => {
                5 + env.current_player().bird_cards.len()
            },
            Action::DiscardBirdCard => {
                env.current_player().bird_cards.len()
            }
            Action::DiscardFood => 5,
            Action::DiscardFoodChoice(choices) => choices.len(),
            Action::DiscardEgg => env.current_player().mat.num_spots_to_discard_eggs(),
            // Do it or not
            Action::DoThen(_, _) => 2,
        }
    }

    pub fn valid_actions(&self, env: &mut WingspanEnv) -> Vec<u8> {
        match self {
            Action::ChooseAction => {
                if Action::PlayBird.is_performable(env) {
                    vec![0, 1, 2, 3]
                } else {
                    vec![1, 2, 3]
                }
            },
            Action::DiscardFoodOrBirdCard => {
                let mut result = Action::DiscardFood.valid_actions(env);
                result.extend(
                    Action::DiscardBirdCard.valid_actions(env).into_iter().map(|idx| 5 + idx)
                );
                result
            },
            Action::DiscardFood => {
                env
                    .current_player()
                    .foods
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, food)| {
                        if *food > 0 {
                            Some(idx as u8)
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            Action::DiscardFoodChoice(choices) => {
                let foods = &env.current_player().foods;
                choices
                    .iter()
                    .filter_map(|(idx, cost)| {
                        if foods[*idx] >= *cost {
                            Some(*idx as u8)
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            Action::PlayBird
                | Action::GetFood
                | Action::GetFoodChoice(_)
                | Action::GetEgg
                | Action::GetBirdCard
                | Action::DiscardBirdCard
                | Action::DiscardEgg
                => {
                (0..self.action_space_size(env) as u8).into_iter().collect()
            },
            Action::DoThen(action_req, _) => {
                if action_req.is_performable(env) {
                    vec![0, 1]
                } else {
                    vec![1]
                }
            }
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyAction {
    inner: Action
}

impl From<Action> for PyAction {
    fn from(inner: Action) -> Self {
        Self { inner }
    }
}

impl From<&Action> for PyAction {
    fn from(inner: &Action) -> Self {
        Self { inner: inner.clone() }
    }
}

#[pymethods]
impl PyAction {
    pub fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}