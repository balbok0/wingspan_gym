use crate::{error::{WingError, WingResult}, habitat::Habitat, wingspan_env::WingspanEnv};


#[derive(Debug, Clone)]
pub enum Action {
    // First decision of the turn (i.e. play a bird, forest, grassland, wetland)
    ChooseAction,

    PlayBird,

    // Get resource actions
    GetFood,
    GetEgg,
    GetBirdCard,

    // Discard Food Or Bird Card
    DiscardFoodOrBirdCard,
    // Discard Bird Card
    DiscardBirdCard,
    // Discard Food
    DiscardFood,
    // Discard Egg
    DiscardEgg,

    // Do something (typically discard to perform an action)
    DoThen(Box<Action>, Box<Action>),
}

impl Action {
    pub fn perform_action(&self, action_idx: u8, env: &mut WingspanEnv) -> WingResult<()> {
        match self {
            Action::DiscardFoodOrBirdCard => {
                env.current_player_mut().discard_food_or_bird_card(action_idx as usize)
            },
            Action::DiscardBirdCard => {
                env.current_player_mut().discard_bird_card(action_idx as usize)
            },
            Action::DiscardFood => {
                env.current_player_mut().discard_food(action_idx as usize)
            },
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
            }
            _ => todo!(),
        }
    }

    pub fn is_performable(&self, env: &mut WingspanEnv) -> bool {
        match self {
            Action::ChooseAction => true,
            Action::PlayBird => env.current_player_mut().can_play_a_bird_card(),
            Action::GetFood => true,
            Action::GetEgg => env.current_player().mat.can_place_egg(),
            Action::GetBirdCard => env.current_player().bird_cards.len() < env.config().hand_limit.into(),
            Action::DiscardFoodOrBirdCard => Action::DiscardFood.is_performable(env) || Action::DiscardBirdCard.is_performable(env),
            Action::DiscardBirdCard => env.current_player().can_discard_bird_card(),
            Action::DiscardFood => env.current_player().can_discard_food(),
            Action::DiscardEgg => env.current_player().mat.can_discard_egg(),
            Action::DoThen(action_req, action_reward) => action_req.is_performable(env) && action_reward.is_performable(env),
        }
    }

    pub fn num_choices(&self, env: &WingspanEnv) -> usize {
        match self {
            Action::ChooseAction => 4,
            Action::PlayBird => {
                env.current_player()._playable_bird_cards.len()
            },
            Action::DiscardFoodOrBirdCard => {
                5 + env.current_player().bird_cards.len()
            },
            Action::DiscardBirdCard => {
                env.current_player().bird_cards.len()
            }
            Action::DiscardFood => 5,
            Action::GetBirdCard => {
                // Implement face up dash
                todo!()
            },
            Action::GetEgg => {
                // Implement egg locations checks in mat
                todo!()
            },
            Action::GetFood => {
                // Implement bird feeder
                todo!()
            }
            Action::DiscardEgg => {
                // Implement egg locations checks in mat
                todo!()
            },
            Action::DoThen(_, _) => 2, // Yes or No
        }
    }
}