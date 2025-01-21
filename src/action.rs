use crate::{error::WingResult, wingspan_env::WingspanEnv};


#[derive(Debug, Clone, Copy)]
pub enum Action {
    // First decision of the turn (i.e. play a bird, forest, grassland, wetland)
    ChooseAction,

    // Discard Food Or Bird Card
    DiscardFoodOrBirdCard,
    // Discard Bird Card
    DiscardBirdCard,
    // Discard Food
    DiscardFood,
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
            _ => todo!(),
        }
    }
}