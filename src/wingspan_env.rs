use std::collections::{HashMap, HashSet};

use derive_builder::Builder;
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    action::{Action, PyAction},
    bird_card::{get_deck as get_birds_deck, BirdCardColor},
    bird_card_callback::BirdCardCallback,
    bird_feeder::BirdFeeder,
    bonus_card::{get_deck as get_bonus_deck, BonusCard},
    deck_and_holder::DeckAndHolder,
    end_of_round_goal::{sample_end_of_round_goals, EndOfRoundGoal, EndOfRoundScoring},
    error::{WingError, WingResult},
    expansion::Expansion,
    food::Foods,
    habitat::{Habitat, HABITATS},
    player::Player,
    step_result::StepResult,
};

#[derive(Debug, Builder, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WingspanEnvConfig {
    #[builder(setter(into), default = 20)]
    pub(crate) hand_limit: u8,
    #[builder(setter(into), default = 2)]
    pub(crate) num_players: usize,
    #[builder(setter(into), default = 4)]
    pub(crate) num_rounds: usize,
    #[builder(default = vec![Expansion::Core])]
    expansions: Vec<Expansion>,
    #[builder(setter(into), default = EndOfRoundScoring::Competitive)]
    scoring_style: EndOfRoundScoring,
}

#[derive(Debug, Clone)]
pub struct WingspanEnv {
    config: WingspanEnvConfig,
    pub(crate) rng: StdRng,
    _round_idx: i8,
    _player_idx: usize,
    _cur_turn_player_idx: usize,
    pub(crate) _bird_deck: DeckAndHolder,
    _bonus_deck: Vec<BonusCard>,
    _end_of_round_goals: Vec<EndOfRoundGoal>,
    _players: Vec<Player>,
    pub(crate) _bird_feeder: BirdFeeder,
    _action_queue: Vec<Action>,
    _callbacks: HashMap<usize, HashSet<BirdCardCallback>>, // List of callback items to go through.
    // List of currently active callbacks (i.e. callbacks - callbacks that already executed)
    _active_callbacks: HashMap<usize, HashSet<BirdCardCallback>>,

    // Whether end of game calculation happened or not
    _end_of_game_happened: bool,

    // Some cards specifically require checking if a predator action succeeds or not.
    // It's a unique dynamic in Wingspan so ok to have this done this way IMO
    pub(crate) _predator_succeeded: bool,

    // Specifically needed for Self::LoggerheadShrike
    // Needs to keep track of state across the turn
    pub(crate) _turn_action_taken: u8,
    pub(crate) _food_at_start_of_turn: Foods,
}

impl WingspanEnv {
    pub fn try_new(config: WingspanEnvConfig) -> Self {
        let num_players = config.num_players;
        let mut env = WingspanEnv {
            config,
            rng: StdRng::from_entropy(),
            // Round index. [0, 3] are normal turns. -1 indicates game setup
            _round_idx: -1,
            // Player currently taking an action
            _player_idx: 0,
            // Player whose turn it currently is
            _cur_turn_player_idx: 0,
            _bird_deck: Default::default(),
            _bonus_deck: Default::default(),
            _bird_feeder: Default::default(),
            _end_of_round_goals: Default::default(),
            _players: Vec::with_capacity(num_players),
            _action_queue: Vec::with_capacity(50), // 50 seems like a reasonable upper bound even for most intense chains?
            _callbacks: Default::default(),
            _active_callbacks: Default::default(),
            _predator_succeeded: false,
            _turn_action_taken: Default::default(),
            _food_at_start_of_turn: Default::default(),
            _end_of_game_happened: false,
        };
        env.reset(None);

        env
    }

    pub fn reset(&mut self, seed: Option<u64>) {
        self._round_idx = -1;
        self._player_idx = 0;

        // If provided reset RNG
        if let Some(seed) = seed {
            self.rng = StdRng::seed_from_u64(seed);
        }

        // Create new deck
        let mut deck = get_birds_deck(&self.config.expansions);
        deck.shuffle(&mut self.rng);
        self._bird_deck = DeckAndHolder::new(deck);
        self._bonus_deck = get_bonus_deck(&self.config.expansions);

        self._end_of_round_goals = sample_end_of_round_goals(
            self.config.expansions.as_slice(),
            self.config.num_rounds,
            &mut self.rng,
        );

        // Give each player cards
        self._players.clear();
        for _player_idx in 0..self.config.num_players {
            let player_bird_cards = self._bird_deck.draw_cards_from_deck(5);
            let player_bonus_cards = self._bonus_deck.split_off(self._bonus_deck.len() - 2);
            self._players
                .push(Player::new(player_bird_cards, player_bonus_cards));
        }

        self._action_queue.clear();
        for _ in 0..5 {
            // Five times make current user decide on what to do
            self.push_action(Action::DiscardFoodOrBirdCard);
        }
        self.push_action(Action::DiscardBonusCard);
    }

    fn post_init_player_setup(&mut self) {
        self._bird_feeder.reroll(&mut self.rng);
        self._bird_deck.reset_display();
    }

    fn start_of_turn(&mut self) {
        self._bird_deck.refill_display();
        self._predator_succeeded = false;

        self._food_at_start_of_turn = *self.current_player().get_foods();
    }

    fn end_of_round(&mut self) -> WingResult<()> {
        self._round_idx += 1;
        self._player_idx = self._round_idx as usize % self.config.num_players;
        self._cur_turn_player_idx = self._player_idx;

        // TODO: End of round abilities

        if self._round_idx > 0 {
            let goal = self._end_of_round_goals[(self._round_idx - 1) as usize];

            self.score_end_of_round_goal(&goal, (self._round_idx - 1) as usize);

            for player_idx in (self._player_idx..self.config.num_players).chain(0..self._player_idx)
            {
                self._player_idx = player_idx;
                let birds = self
                    .get_player(player_idx)
                    .get_birds_on_mat()
                    .iter()
                    .zip(HABITATS)
                    .flat_map(|(v, hab)| v.iter().map(move |bc| (*bc, hab)).enumerate())
                    .filter(|(_, (bc, _))| *bc.color() == BirdCardColor::Teal)
                    .collect::<Vec<_>>();
                for (bird_idx, (bc, habitat)) in birds.clone() {
                    bc.activate(self, &habitat, bird_idx)?;
                }
            }

            self._player_idx = self._cur_turn_player_idx;
        }

        // Start of the new round
        for player in self._players.iter_mut() {
            player.set_turns_left(8 - self._round_idx as u8);
        }
        self._bird_deck.reset_display();

        Ok(())
    }

    fn end_of_game(&mut self) -> WingResult<()> {
        if self._end_of_game_happened {
            return Err(WingError::TaskOutOfOrder(
                "end of game sequence called multiple times".to_string(),
            ));
        }
        if self._round_idx != self.config.num_rounds as i8 {
            return Err(WingError::TaskOutOfOrder(format!(
                "end of game sequence called on round {}, but game ends on round {}",
                self._round_idx, self.config.num_rounds
            )));
        }

        // Activate them birds
        for player_idx in 0..self.config.num_players {
            self._player_idx = player_idx;
            let birds = self
                .get_player(player_idx)
                .get_birds_on_mat()
                .iter()
                .zip(HABITATS)
                .flat_map(|(v, hab)| v.iter().map(move |bc| (*bc, hab)).enumerate())
                .filter(|(_, (bc, _))| *bc.color() == BirdCardColor::Yellow)
                .collect::<Vec<_>>();
            for (bird_idx, (bc, habitat)) in birds.clone() {
                bc.activate(self, &habitat, bird_idx)?;
            }
        }
        self._end_of_game_happened = true;

        Ok(())
    }

    pub fn points(&self) -> Vec<u8> {
        self._players
            .iter()
            .map(|p| p.calculate_points())
            .collect_vec()
    }

    fn check_callbacks(&mut self, action: &Action, action_idx: u8) -> WingResult<()> {
        for player_idx in 0..self.config.num_players {
            if player_idx == self._cur_turn_player_idx {
                continue;
            }

            let mut callbacks_to_remove = vec![];
            if let Some(callbacks) = self._active_callbacks.get(&player_idx) {
                // This clone hurts. Maybe there is a better way to do it?
                for callback in callbacks.clone().iter() {
                    let callback_successful = callback.card.conditional_callback(
                        self,
                        action,
                        action_idx,
                        &callback.habitat,
                        callback.card_idx,
                        callback.card_player_idx,
                    )?;

                    if callback_successful {
                        callbacks_to_remove.push(callback.clone())
                    }
                }
            }

            if let Some(callbacks) = self._active_callbacks.get_mut(&player_idx) {
                for cb in callbacks_to_remove {
                    callbacks.remove(&cb);
                    if cb.card.is_predator() {
                        self._predator_succeeded = true;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn step(&mut self, action_idx: u8) -> WingResult<StepResult> {
        if self._round_idx == 4 {
            // We have terminated / End of round
            return Ok(StepResult::Terminated);
        }

        // unwrap is safe, since there is a check in the end
        let action = self._action_queue.last().unwrap().clone();
        if !action.is_performable(self) {
            return Err(WingError::InvalidAction);
        }
        let mut action = self._action_queue.pop().unwrap();
        if let Err(e) = action.perform_action(action_idx, self) {
            self.push_action(action);
            return Err(e);
        };

        // Update action here to keep track of what is going on this turn
        if matches!(action, Action::ChooseAction) {
            self._turn_action_taken = action_idx;
        }
        self.check_callbacks(&action, action_idx)?;

        // Ensure that next action can be performed
        while !self._action_queue.is_empty() {
            let next_action = self._action_queue.last().unwrap().clone();

            // If next action is not performable, remove it
            if !next_action.is_performable(self) {
                self._action_queue.pop();
                continue;
            }

            // If next action has only one valid action, just do it
            let valid_actions = next_action.valid_actions(self);
            if valid_actions.len() == 1 {
                self.step(valid_actions[0])?;
            } else {
                // Next action is valid and has more than one option
                break;
            }
        }

        // Handle end of turn for the player
        if self._action_queue.is_empty() {
            // Re-activate current players callbacks
            self._active_callbacks.entry(self._player_idx).insert_entry(
                self._callbacks
                    .get(&self._player_idx)
                    .cloned()
                    .unwrap_or(HashSet::new()),
            );

            // Go to next player
            self.increment_player_idx();

            // Clear their callbacks
            self._active_callbacks
                .entry(self._player_idx)
                .and_modify(|v| v.clear());

            // Special case is first round
            if self._round_idx == -1 {
                if self._player_idx == 0 {
                    // Setup is done from players side.
                    self.post_init_player_setup();

                    // TODO: Finish it here and then make it round 0
                    self._round_idx = 0;
                    self._cur_turn_player_idx = 0;
                    self._player_idx = 0;
                    for player in self._players.iter_mut() {
                        player.set_turns_left(8);
                    }
                    self.push_action(Action::ChooseAction);
                    // Reduce number of turns left, since a new player will be making a move
                    self.current_player_mut().turns_left -= 1;
                } else {
                    // Next player can do setup
                    for _ in 0..5 {
                        // Five times make current user decide on what to do
                        self.push_action(Action::DiscardFoodOrBirdCard);
                    }
                }
            } else {
                self._player_idx %= self.config.num_players;
                // Normal rounds
                if self.current_player().turns_left == 0 {
                    // End of round
                    self.end_of_round()?;

                    if self._round_idx == self.config.num_rounds as i8 {
                        // End of game is after Round 4 (0 - when it is zero indexed)
                        self.end_of_game()?;
                        return Ok(StepResult::Terminated);
                    }
                } else {
                    // End of normal turn
                    self.start_of_turn();
                }
                self.push_action(Action::ChooseAction);

                // Reduce number of turns left, since a new player will be making a move
                self.current_player_mut().turns_left -= 1;
            }
        }

        Ok(StepResult::Live)
    }

    fn score_end_of_round_goal(&mut self, goal: &EndOfRoundGoal, round_to_score_idx: usize) {
        // TODO: Implement also non-competitive scoring
        let scores = (0..self._players.len())
            .map(|player_idx| (goal.get_num_matching(self, player_idx), player_idx))
            .sorted();

        const COMPETITIVE_BASE_SCORES: [u8; 4] = [4, 1, 0, 0];
        const COMPETITIVE_PER_ROUND_SCORE_ADDS: [u8; 4] = [1, 1, 1, 0];

        let mut cur_score = 0;
        let mut cur_players = vec![];

        match self.config.scoring_style {
            EndOfRoundScoring::Competitive => {
                for (score_idx, (player_score, player_idx)) in scores.enumerate() {
                    // First iter
                    if score_idx == 0 {
                        cur_score = player_score;
                        cur_players.push(player_idx);
                        continue;
                    }

                    // 0-score is always worth nothing
                    if cur_score == 0 {
                        break;
                    }

                    // More than 1 player with the same score
                    if player_score == cur_score {
                        cur_players.push(player_idx);
                        continue;
                    }

                    // At this point scores are different

                    // First process current queue
                    let cur_score_idx = score_idx - cur_players.len();
                    let total_pts: u8 =
                        // Points from base score
                        COMPETITIVE_BASE_SCORES[cur_score_idx..score_idx.min(4)].iter().sum::<u8>() +
                        // They increase per round
                        COMPETITIVE_PER_ROUND_SCORE_ADDS[cur_score_idx..score_idx.min(4)].iter().sum::<u8>() * (round_to_score_idx as u8)
                    ;
                    let pts_per_player = total_pts / cur_players.len() as u8;
                    for cur_player_idx in cur_players.iter() {
                        self.get_player_mut(*cur_player_idx)
                            .add_end_of_round_points(pts_per_player);
                    }

                    // Only up-to third place gets points.
                    if score_idx > 2 {
                        break;
                    }

                    // Keep track of new score and new player set
                    cur_players.clear();
                    cur_score = player_score;
                    cur_players.push(player_idx);
                }
            }
            EndOfRoundScoring::Friendly => {
                for (player_score, player_idx) in scores {
                    self.get_player_mut(player_idx)
                        .add_end_of_round_points(player_score.min(5) as u8);
                }
            }
        }
    }

    pub fn populate_action_queue_from_habitat_action(&mut self, habitat: &Habitat) {
        let mut actions = self
            .current_player_mut()
            .get_mat_mut()
            .get_actions_from_habitat_action(habitat);

        self.append_actions(&mut actions);
    }

    pub fn draw_bonus_cards(&mut self, num_cards: usize) {
        let mut player_bonus_cards = self
            ._bonus_deck
            .split_off(self._bonus_deck.len() - num_cards);

        self.current_player_mut()
            .add_bonus_cards(&mut player_bonus_cards);
    }

    pub fn get_player(&self, player_idx: usize) -> &Player {
        &self._players[player_idx % self._players.len()]
    }

    pub fn get_player_mut(&mut self, player_idx: usize) -> &mut Player {
        let player_idx = player_idx % self._players.len();
        &mut self._players[player_idx]
    }

    pub fn current_player(&self) -> &Player {
        &self._players[self._player_idx]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self._players[self._player_idx]
    }

    pub fn current_player_idx(&self) -> usize {
        self._player_idx
    }

    pub fn current_turn_player(&self) -> &Player {
        &self._players[self._cur_turn_player_idx]
    }

    pub fn current_turn_player_mut(&mut self) -> &mut Player {
        &mut self._players[self._cur_turn_player_idx]
    }

    pub fn current_turn_player_idx(&self) -> usize {
        self._cur_turn_player_idx
    }

    pub fn set_current_player(&mut self, idx: usize) {
        self._player_idx = idx;
    }

    pub fn increment_player_idx(&mut self) {
        self._cur_turn_player_idx += 1;
        self._cur_turn_player_idx %= self.config.num_players;

        self._player_idx = self._cur_turn_player_idx;
    }

    pub fn action_space_size(&self) -> Option<usize> {
        self._action_queue.last().map(|x| x.action_space_size(self))
    }

    pub fn config(&self) -> &WingspanEnvConfig {
        &self.config
    }

    pub fn next_action(&self) -> Option<&Action> {
        self._action_queue.last()
    }

    pub fn push_action(&mut self, action: Action) {
        self._action_queue.push(action)
    }

    pub fn append_actions(&mut self, actions: &mut Vec<Action>) {
        // Appends actions to the top of the stack (it is a LIFO queue)
        // Note that hence last element of actions will become `env.next_action`
        self._action_queue.append(actions)
    }

    pub fn prepend_actions(&mut self, actions: &mut [Action]) {
        // Appends actions to the bottom of the stack (it is a LIFO queue)
        // Note that hence first element of actions will become the last action to be taken this round
        self._action_queue
            .splice(..0, actions.iter_mut().map(|x| x.clone()));
    }

    pub fn push_callback(&mut self, callback: BirdCardCallback) {
        self._callbacks
            .entry(callback.card_player_idx)
            .or_default()
            .insert(callback);
    }

    pub fn predator_succeeded(&mut self) {
        self._predator_succeeded = true;
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyWingspanEnv {
    inner: WingspanEnv,
}

#[pymethods]
impl PyWingspanEnv {
    #[new]
    #[pyo3(signature = (hand_limit=None, num_players=None))]
    pub fn new(hand_limit: Option<u8>, num_players: Option<u8>) -> PyResult<Self> {
        let mut builder = &mut WingspanEnvConfigBuilder::create_empty();
        if let Some(hand_limit) = hand_limit {
            builder = builder.hand_limit(hand_limit);
        }
        if let Some(num_players) = num_players {
            builder = builder.num_players(num_players);
        }
        let config = builder
            .build()
            .map_err(|err| PyValueError::new_err(format!("Error building config: {err}")))?;

        Ok(Self {
            inner: WingspanEnv::try_new(config),
        })
    }

    #[getter]
    pub fn player_idx(slf: &Bound<'_, Self>) -> usize {
        slf.borrow().inner._player_idx
    }

    #[getter]
    pub fn round_idx(slf: &Bound<'_, Self>) -> i8 {
        slf.borrow().inner._round_idx
    }

    #[pyo3(signature = (seed=None))]
    pub fn reset(slf: &Bound<'_, Self>, seed: Option<u64>) {
        slf.borrow_mut().inner.reset(seed)
    }

    pub fn step(slf: &Bound<'_, Self>, action_idx: u8) -> PyResult<StepResult> {
        match slf.borrow_mut().inner.step(action_idx) {
            Ok(x) => Ok(x),
            Err(WingError::InvalidAction) => Ok(StepResult::Invalid),
            Err(x) => Err(x.into()),
            // Err(x) => return Err(x.into()),
        }
    }

    pub fn points(&self) -> Vec<usize> {
        self.inner
            .points()
            .into_iter()
            .map(|v| v as usize)
            .collect_vec()
    }

    pub fn action_space_size(slf: &Bound<'_, Self>) -> Option<usize> {
        slf.borrow().inner.action_space_size()
    }

    #[allow(clippy::type_complexity)]
    pub fn _debug_get_state(
        slf: &Bound<'_, Self>,
    ) -> (
        i8,
        usize,
        Option<String>,
        Vec<Player>,
        HashMap<usize, HashSet<BirdCardCallback>>,
    ) {
        let inner = &slf.borrow().inner;

        (
            inner._round_idx,
            inner._player_idx,
            inner._action_queue.last().map(|x| format!("{x:?}")),
            inner._players.clone(),
            inner._active_callbacks.clone(),
        )
    }

    pub fn next_action(slf: &Bound<'_, Self>) -> Option<PyAction> {
        slf.borrow().inner.next_action().map(PyAction::from)
    }
}
