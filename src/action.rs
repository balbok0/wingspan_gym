use crate::{
    bird_card::{BirdCard, BirdCardColor},
    error::{WingError, WingResult},
    food::FoodIndex,
    habitat::Habitat,
    wingspan_env::WingspanEnv,
};
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    // First decision of the turn (i.e. play a bird, forest, grassland, wetland)
    #[allow(clippy::enum_variant_names)]
    ChooseAction,
    BirdActionFromHabitat(Habitat),

    PlayBird,
    PlayBirdHabitat(Habitat),

    // Get resource actions
    GetFood,
    GetFoodFromSupplyChoice(Box<[FoodIndex]>),
    GetEgg,
    GetEggAtLoc(Habitat, usize, usize),
    GetEggChoice(Box<[(Habitat, usize)]>, EggCapacityOverride),
    GetBirdCard,
    GetBirdCardFromDeck,

    // Discard actions,
    DiscardFoodOrBirdCard,
    DiscardBirdCard,
    TuckBirdCard(Habitat, usize),
    TuckBirdCardFromDeck(Habitat, usize),
    DiscardBonusCard,
    DiscardFood,
    DiscardFoodChoice(Box<[(FoodIndex, u8)]>), // Discard food of choice N times
    DiscardEgg,
    DiscardEggChoice(Box<[(Habitat, usize)]>),
    // Cache food of choice N times on specific bird.
    CacheFoodChoice(Box<[(FoodIndex, u8)]>, Habitat, usize),

    // Wrapper actions
    DoThen(Box<Action>, Box<Action>),
    Option(Box<Action>),
    MultipleActions(Vec<Action>),

    // Special cases

    // Move a bird from one habitat to another. It's for cards with "when this card is rightmost in X"
    MoveBird(BirdCard, Vec<Habitat>),

    // Change Player allows for a temporary change of a player.
    // This typically sandwiches 1 action with Change Player to temp -> Do Action -> Change Player back
    ChangePlayer(usize),

    // Allows to choose an "index" of a thing (typically a starting player, or a bird to copy something from)
    // It is tied to the BirdCard::after_choice_callback function
    // First field size size of action choice (i.e. num of choices, and everything else is what is needed to call the callback)
    #[allow(clippy::enum_variant_names)]
    ChooseThenAction(u8, BirdCard, Habitat, usize),

    // From a specified set of cards, get a card by index and increase idx of a player
    // This occurs in AmericanOystercatcher case, and written for that case
    GetCardFromSetAndChangePlayer(Vec<BirdCard>),
}

impl Action {
    pub fn perform_action(&mut self, action_idx: u8, env: &mut WingspanEnv) -> WingResult<()> {
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
                            return Err(WingError::InvalidAction);
                        }

                        env.push_action(Action::PlayBird);
                        return Ok(());
                    }
                    _ => return Err(WingError::InvalidAction),
                };

                env.populate_action_queue_from_habitat_action(&habitat);

                Ok(())
            }
            Action::BirdActionFromHabitat(habitat) => {
                let mat_row = env.current_player().get_mat().get_row(habitat).clone();
                let (mut actions, mut end_of_turn_actions) = mat_row.get_bird_actions(env);
                // TODO: Actions here should be empty I think
                env.append_actions(&mut end_of_turn_actions);
                env.append_actions(&mut actions);

                Ok(())
            }
            Action::PlayBird | Action::PlayBirdHabitat(_) => {
                let (bird_card, habitat, bird_idx, mut followup_actions) =
                    env.current_player_mut().play_a_bird_card(action_idx)?;

                if bird_card.color() == &BirdCardColor::White {
                    // TODO: After all (from core) of the BirdCard actions are implemented, do uncomment below
                    let mut action_result = bird_card.activate(env, &habitat, bird_idx).unwrap();
                    env.append_actions(&mut action_result.end_of_turn_actions);
                    env.append_actions(&mut action_result.immediate_actions);
                }

                env.append_actions(&mut followup_actions);
                Ok(())
            }
            Action::GetFood => {
                match env._bird_feeder.take_dice_and_update_state(
                    &mut env.rng,
                    action_idx,
                    self.clone(),
                )? {
                    crate::bird_feeder::BirdFeederActionResult::GainFood(food_idx) => {
                        env.current_player_mut().add_food(food_idx, 1)
                    }
                    crate::bird_feeder::BirdFeederActionResult::FollowupAction(action) => {
                        env.push_action(action)
                    }
                }
                Ok(())
            }
            Action::GetFoodFromSupplyChoice(choices) => {
                let action_idx = action_idx as usize;
                if action_idx >= choices.len() {
                    Err(WingError::InvalidAction)
                } else {
                    env.current_player_mut().add_food(choices[action_idx], 1);
                    Ok(())
                }
            }
            Action::GetEgg => env.current_player_mut().get_mat_mut().place_egg(action_idx),
            Action::GetEggAtLoc(habitat, bird_idx, num_eggs) => {
                for _ in 0..*num_eggs {
                    // Ignore errors, since some eggs might have succeeded. If user makes bad moves let them
                    let _ = env
                        .current_player_mut()
                        .get_mat_mut()
                        .get_row_mut(habitat)
                        .place_egg_at_exact_bird_idx(*bird_idx, 0);
                }

                Ok(())
            }
            Action::GetEggChoice(choices, egg_cap_override) => {
                let action_idx = action_idx as usize;
                if action_idx >= choices.len() {
                    Err(WingError::InvalidAction)
                } else {
                    let (habitat, bird_idx) = choices[action_idx];
                    // TODO: Potentially check here if needed.
                    env.current_player_mut()
                        .get_mat_mut()
                        .get_row_mut(&habitat)
                        .place_egg_at_exact_bird_idx(bird_idx, (*egg_cap_override).into())
                }
            }
            Action::GetBirdCard => {
                let card = env._bird_deck.draw_card(action_idx)?;
                env.current_player_mut().add_bird_card(card);
                Ok(())
            }
            Action::GetBirdCardFromDeck => {
                let card = env._bird_deck.draw_cards_from_deck(1)[0];
                env.current_player_mut().add_bird_card(card);
                Ok(())
            }
            Action::DiscardFoodOrBirdCard => env
                .current_player_mut()
                .discard_food_or_bird_card(action_idx as usize),
            Action::DiscardBirdCard => env
                .current_player_mut()
                .discard_bird_card(action_idx as usize),
            Action::TuckBirdCard(habitat, bird_idx) => {
                env.current_player_mut()
                    .discard_bird_card(action_idx as usize)?;
                env.current_player_mut()
                    .get_mat_mut()
                    .get_row_mut(habitat)
                    .tuck_card(*bird_idx);

                Ok(())
            }
            Action::TuckBirdCardFromDeck(habitat, bird_idx) => {
                env.current_player_mut()
                    .get_mat_mut()
                    .get_row_mut(habitat)
                    .tuck_card(*bird_idx);

                Ok(())
            }
            Action::DiscardBonusCard => env
                .current_player_mut()
                .discard_bonus_card(action_idx as usize),
            Action::DiscardFood => {
                if action_idx >= 5 {
                    return Err(WingError::InvalidAction);
                }
                env.current_player_mut()
                    .discard_food(FoodIndex::from(action_idx), 1)
            }
            Action::DiscardFoodChoice(choices) => {
                let (food_idx, num_food) = choices
                    .get(action_idx as usize)
                    .ok_or(WingError::InvalidAction)?;

                env.current_player_mut().discard_food(*food_idx, *num_food)
            }
            Action::DiscardEgg => env
                .current_player_mut()
                .get_mat_mut()
                .discard_egg(action_idx),
            Action::DiscardEggChoice(choices) => {
                let action_idx = action_idx as usize;
                if action_idx >= choices.len() {
                    Err(WingError::InvalidAction)
                } else {
                    let (habitat, bird_idx) = choices[action_idx];
                    env.current_player_mut()
                        .get_mat_mut()
                        .get_row_mut(&habitat)
                        .discard_egg_at_exact_bird_idx(bird_idx)
                }
            }
            Action::CacheFoodChoice(foods, habitat, bird_idx) => {
                let (food_index, num_food) = foods
                    .get(action_idx as usize)
                    .ok_or(WingError::InvalidAction)?;
                let row = env.current_player_mut().get_mat_mut().get_row_mut(habitat);
                for _ in 0..*num_food {
                    row.cache_food(*bird_idx, *food_index);
                }
                Ok(())
            }
            Action::MoveBird(bird_card, habitats) => {
                let target_habitat = habitats
                    .get(action_idx as usize)
                    .ok_or(WingError::InvalidAction)?;
                env.current_player_mut()
                    .get_mat_mut()
                    .move_bird(*bird_card, *target_habitat)
            }
            Action::DoThen(a, b) => {
                match action_idx {
                    0 => {
                        // Option rejected
                        Ok(())
                    }
                    1 => {
                        // Option accepted
                        env.push_action(*b.clone());
                        env.push_action(*a.clone());
                        Ok(())
                    }
                    _ => Err(WingError::InvalidAction),
                }
            }
            Action::Option(a) => match action_idx {
                0 => Ok(()),
                1 => {
                    env.push_action(*a.clone());
                    Ok(())
                }
                _ => Err(WingError::InvalidAction),
            },
            Action::MultipleActions(actions) => {
                env.append_actions(actions);
                Ok(())
            }
            Action::ChangePlayer(player_idx) => {
                if *player_idx >= env.config().num_players {
                    return Err(WingError::InvalidAction);
                }
                env.set_current_player(*player_idx);
                Ok(())
            }
            Action::ChooseThenAction(choice_size, bird_card, habitat, bird_idx) => {
                if action_idx >= *choice_size {
                    return Err(WingError::InvalidAction);
                }
                let mut action_result =
                    bird_card.after_choice_callback(action_idx, env, habitat, *bird_idx)?;
                env.append_actions(&mut action_result.end_of_turn_actions);
                env.append_actions(&mut action_result.immediate_actions);
                Ok(())
            }
            Action::GetCardFromSetAndChangePlayer(cards) => {
                let action_idx = action_idx as usize;
                if action_idx >= cards.len() {
                    return Err(WingError::InvalidAction);
                }

                env.current_player_mut()
                    .add_bird_card(cards.remove(action_idx));
                env.increment_player_idx();

                if !cards.is_empty() {
                    env.push_action(Action::GetCardFromSetAndChangePlayer(cards.to_owned()));
                }
                Ok(())
            }
            // x => {
            //     println!("Action not implemented: {:?}", x);
            //     todo!()
            // },
        }
    }

    pub fn is_performable(&self, env: &mut WingspanEnv) -> bool {
        match self {
            Action::ChooseAction => true,
            Action::BirdActionFromHabitat(_) => true,
            Action::PlayBird => env.current_player_mut().can_play_a_bird_card(vec![
                Habitat::Forest,
                Habitat::Grassland,
                Habitat::Wetland,
            ]),
            Action::PlayBirdHabitat(habitat) => env
                .current_player_mut()
                .can_play_a_bird_card(vec![*habitat]),
            Action::GetFood => true,
            Action::GetFoodFromSupplyChoice(_) => true,
            Action::GetEgg => env.current_player().get_mat().can_place_egg(),
            Action::GetEggAtLoc(_, _, _) => self.action_space_size(env) > 0,
            Action::GetEggChoice(_, _) => !self.valid_actions(env).is_empty(),
            Action::GetBirdCard | Action::GetBirdCardFromDeck => {
                env.current_player().get_bird_cards().len() < env.config().hand_limit.into()
            }
            Action::DiscardFoodOrBirdCard => {
                Action::DiscardFood.is_performable(env)
                    || Action::DiscardBirdCard.is_performable(env)
            }
            Action::DiscardBirdCard | Action::TuckBirdCard(_, _) => {
                env.current_player().can_discard_bird_card()
            }
            Action::DiscardBonusCard => !env.current_player().get_bonus_cards().is_empty(),
            Action::DiscardFood => env.current_player().can_discard_food(),
            Action::DiscardFoodChoice(choices) => {
                let foods = env.current_player().get_foods();
                choices
                    .iter()
                    .map(|(idx, cost)| foods[*idx as usize] >= *cost)
                    .reduce(|a, b| a || b)
                    .unwrap_or(true)
            }
            Action::DiscardEgg => env.current_player().get_mat().can_discard_egg(),
            Action::DiscardEggChoice(_) => !self.valid_actions(env).is_empty(),
            Action::TuckBirdCardFromDeck(_, _) => true,
            Action::CacheFoodChoice(_, _, _) => true,
            Action::MoveBird(_, _) => self.action_space_size(env) > 0,
            Action::DoThen(action_req, action_reward) => {
                action_req.is_performable(env) && action_reward.is_performable(env)
            }
            Action::Option(action) => action.is_performable(env),
            Action::MultipleActions(_) => true,
            Action::ChangePlayer(player_idx) => *player_idx < env.config().num_players,
            Action::ChooseThenAction(choice_size, _, _, _) => *choice_size > 0,
            Action::GetCardFromSetAndChangePlayer(cards) => !cards.is_empty(),
        }
    }

    pub fn action_space_size(&self, env: &WingspanEnv) -> usize {
        match self {
            Action::ChooseAction => 4,
            Action::BirdActionFromHabitat(_) => 1,
            Action::PlayBird => env.current_player().get_playable_card_hab_combos().len(),
            Action::PlayBirdHabitat(_) => {
                // Note: card habitat combos are populated with only that habitat
                env.current_player().get_playable_card_hab_combos().len()
            }
            Action::GetFood => env._bird_feeder.num_actions(),
            Action::GetFoodFromSupplyChoice(choices) => choices.len(),
            Action::GetEgg => env.current_player().get_mat().num_spots_to_place_eggs(),
            Action::GetEggAtLoc(habitat, bird_idx, _) => {
                let mat_row = env.current_player().get_mat().get_row(habitat);
                let bird_idx = *bird_idx;

                if mat_row.get_eggs()[bird_idx] < mat_row.get_eggs_cap()[bird_idx] {
                    1
                } else {
                    0
                }
            }
            Action::GetEggChoice(choices, _) => choices.len(),
            Action::GetBirdCard => env._bird_deck.num_actions(),
            Action::GetBirdCardFromDeck => 1,
            Action::DiscardFoodOrBirdCard => 5 + env.current_player().get_bird_cards().len(),
            Action::DiscardBirdCard | Action::TuckBirdCard(_, _) => {
                env.current_player().get_bird_cards().len()
            }
            Action::DiscardBonusCard => env.current_player().get_bonus_cards().len(),
            Action::TuckBirdCardFromDeck(_, _) => 1,
            Action::DiscardFood => 5,
            Action::DiscardFoodChoice(choices) => choices.len(),
            Action::DiscardEgg => env.current_player().get_mat().num_spots_to_discard_eggs(),
            Action::DiscardEggChoice(choices) => choices.len(),
            Action::CacheFoodChoice(food_choices, _, _) => food_choices.len(),
            Action::MoveBird(bird_card, habitats) => env
                .current_player()
                .get_mat()
                .playable_habitats(bird_card)
                .iter()
                .filter(|hab| habitats.contains(hab))
                .count(),
            // Do it or not
            Action::DoThen(_, _) => 2,
            Action::Option(_) => 2,
            Action::MultipleActions(_) => 1,
            Action::ChangePlayer(_) => 1,
            Action::ChooseThenAction(choice_size, _, _, _) => *choice_size as usize,
            Action::GetCardFromSetAndChangePlayer(cards) => cards.len(),
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
            }
            Action::BirdActionFromHabitat(_) => vec![0],
            Action::DiscardFoodOrBirdCard => {
                let mut result = Action::DiscardFood.valid_actions(env);
                result.extend(
                    Action::DiscardBirdCard
                        .valid_actions(env)
                        .into_iter()
                        .map(|idx| 5 + idx),
                );
                result
            }
            Action::DiscardFood => env
                .current_player()
                .get_foods()
                .iter()
                .enumerate()
                .filter_map(|(idx, food)| if *food > 0 { Some(idx as u8) } else { None })
                .collect(),
            Action::DiscardFoodChoice(choices) => {
                let foods = &env.current_player().get_foods();
                choices
                    .iter()
                    .filter_map(|(idx, cost)| {
                        let idx = *idx;
                        if foods[idx as usize] >= *cost {
                            Some(idx as u8)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            Action::GetEggChoice(choices, egg_cap_override) => choices
                .iter()
                .enumerate()
                .filter_map(|(choice_idx, (habitat, bird_idx))| {
                    if env
                        .current_player()
                        .get_mat()
                        .get_row(habitat)
                        .can_place_egg(*bird_idx, egg_cap_override.into())
                    {
                        Some(choice_idx as u8)
                    } else {
                        None
                    }
                })
                .collect(),
            Action::DiscardEggChoice(choices) => choices
                .iter()
                .enumerate()
                .filter_map(|(choice_idx, (habitat, bird_idx))| {
                    if env
                        .current_player()
                        .get_mat()
                        .get_row(habitat)
                        .can_discard_egg(*bird_idx)
                    {
                        Some(choice_idx as u8)
                    } else {
                        None
                    }
                })
                .collect(),
            Action::MoveBird(_, habs) => (0..habs.len() as u8).collect(),
            Action::PlayBird
            | Action::PlayBirdHabitat(_)
            | Action::GetFood
            | Action::GetFoodFromSupplyChoice(_)
            | Action::GetEgg
            | Action::GetBirdCard
            | Action::GetBirdCardFromDeck
            | Action::DiscardBirdCard
            | Action::TuckBirdCard(_, _)
            | Action::TuckBirdCardFromDeck(_, _)
            | Action::GetEggAtLoc(_, _, _)
            | Action::DiscardBonusCard
            | Action::DiscardEgg
            | Action::CacheFoodChoice(_, _, _)
            | Action::MultipleActions(_)
            | Action::ChangePlayer(_)
            | Action::GetCardFromSetAndChangePlayer(_) => {
                (0..self.action_space_size(env) as u8).collect()
            }
            Action::DoThen(action, _) | Action::Option(action) => {
                if action.is_performable(env) {
                    vec![0, 1]
                } else {
                    vec![0]
                }
            }
            Action::ChooseThenAction(choice_size, _, _, _) => (0..*choice_size).collect(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyAction {
    inner: Action,
}

impl From<Action> for PyAction {
    fn from(inner: Action) -> Self {
        Self { inner }
    }
}

impl From<&Action> for PyAction {
    fn from(inner: &Action) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[pymethods]
impl PyAction {
    pub fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

// Helper things for Specific actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EggCapacityOverride {
    None,
    Over(u8),
}

impl Default for EggCapacityOverride {
    fn default() -> Self {
        Self::None
    }
}

impl From<EggCapacityOverride> for u8 {
    fn from(value: EggCapacityOverride) -> Self {
        match value {
            EggCapacityOverride::None => 0,
            EggCapacityOverride::Over(val) => val,
        }
    }
}

impl From<&EggCapacityOverride> for u8 {
    fn from(value: &EggCapacityOverride) -> Self {
        match value {
            EggCapacityOverride::None => 0,
            EggCapacityOverride::Over(val) => *val,
        }
    }
}
