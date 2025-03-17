use derive_builder::Builder;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    action::{Action, PyAction}, bird_card::get_deck as get_birds_deck, bird_card_callback::BirdCardCallback, bird_feeder::BirdFeeder, bonus_card::{get_deck as get_bonus_deck, BonusCard}, deck_and_holder::DeckAndHolder, error::{WingError, WingResult}, expansion::Expansion, habitat::Habitat, player::Player, step_result::StepResult
};

#[derive(Debug, Builder, Clone)]
pub struct WingspanEnvConfig {
    #[builder(setter(into), default = 20)]
    pub(crate) hand_limit: u8,
    #[builder(setter(into), default = 2)]
    pub(crate) num_players: usize,
    #[builder(default = vec![Expansion::Core])]
    expansions: Vec<Expansion>,
}


#[derive(Debug, Clone)]
pub struct WingspanEnv {
    config: WingspanEnvConfig,
    pub(crate) rng: StdRng,
    _round_idx: i8,
    _player_idx: usize,
    pub(crate) _bird_deck: DeckAndHolder,
    _bonus_deck: Vec<BonusCard>,
    _players: Vec<Player>,
    pub(crate) _bird_feeder: BirdFeeder,
    _action_queue: Vec<Action>,
    _callbacks: Vec<BirdCardCallback>,  // List of callback items to go through.
}

impl WingspanEnv {
    pub fn try_new(config: WingspanEnvConfig) -> Self {
        let num_players = config.num_players;
        let mut env = WingspanEnv {
            config,
            rng: StdRng::from_entropy(),
            _round_idx: -1,
            _player_idx: 0,
            _bird_deck: Default::default(),
            _bonus_deck: Default::default(),
            _bird_feeder: Default::default(),
            _players: Vec::with_capacity(num_players),
            _action_queue: Vec::with_capacity(50), // 50 seems like a reasonable upper bound even for most intense chains?
            _callbacks: Default::default(),
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

        // TODO: Bonus cards for players

        // Give each player foods

        for _ in 0..self.config.num_players {
            let player_bird_cards = self._bird_deck.draw_cards_from_deck(5);
            let player_bonus_cards = self._bonus_deck.split_off(self._bonus_deck.len() - 2);
            self._players.push(Player::new(player_bird_cards, player_bonus_cards));
        }

        self._action_queue.clear();
        for _ in 0..5 {
            // Five times make current user decide on what to do
            self.push_action(Action::DiscardFoodOrBirdCard);
        }
        self.push_action(Action::DiscardBirdCard);

        // TODO: 3 birds face-up
    }

    fn post_init_player_setup(&mut self) {
        self._bird_feeder.reroll(&mut self.rng);
        self._bird_deck.reset_display();
    }

    fn end_of_turn(&mut self) {
        self._bird_deck.refill_display();
    }

    fn end_of_round(&mut self) {
        self._round_idx += 1;
        self._player_idx = self._round_idx as usize % self.config.num_players;

        // TODO: End of round abilities
        // TODO: End of round goals

        // Start of the new round
        for player in self._players.iter_mut() {
            player.set_turns_left(8 - self._round_idx as u8);
        }
        self._bird_deck.reset_display();
    }

    pub fn step(&mut self, action_idx: u8) -> WingResult<StepResult> {
        if self._round_idx == 4 {
            println!("Action queue: {:?}", self._action_queue);
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

        // Ensure that next action can be performed
        while !self._action_queue.is_empty() {
            let next_action = self._action_queue.last().unwrap().clone();

            // If next action is not performable, remove it
            if !next_action.is_performable(self) {
                self._action_queue.pop();
                continue;
            }

            // If next action has only one valid action, just do it
            let valid_actions = next_action.valid_actions( self);
            if valid_actions.len() == 1 {
                self.step(valid_actions[0])?;
            } else {
                // Next action is valid and has more than one option
                break;
            }
        }

        // Handle end of turn for the player
        if self._action_queue.is_empty() {
            // Loop through players
            self._player_idx += 1;

            // Special case is first round
            if self._round_idx == -1 {
                if self._player_idx == self.config.num_players {
                    // Setup is done from players side.
                    self.post_init_player_setup();

                    // TODO: Finish it here and then make it round 0
                    self._round_idx = 0;
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
                    self.end_of_round();

                    if self._round_idx == 4 {
                        // End of game is after Round 4 (0 - when it is zero indexed)
                        return Ok(StepResult::Terminated);
                    }
                } else {
                    // End of normal turn
                    self.end_of_turn();
                }
                self.push_action(Action::ChooseAction);

                // Reduce number of turns left, since a new player will be making a move
                self.current_player_mut().turns_left -= 1;
            }
        }

        Ok(StepResult::Live)
    }

    pub fn populate_action_queue_from_habitat_action(&mut self, habitat: &Habitat) {
        let mut actions = self.current_player_mut().get_mat_mut().get_actions_from_habitat_action(habitat);

        self.append_actions(&mut actions);
    }

    pub fn draw_bonus_cards(&mut self, num_cards: usize) {
        let mut player_bonus_cards = self._bonus_deck.split_off(self._bonus_deck.len() - num_cards);

        self.current_player_mut().add_bonus_cards(&mut player_bonus_cards);
    }

    pub fn get_player(&self, player_idx: usize) -> &Player {
        &self._players[player_idx % self._players.len()]
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

    pub fn set_current_player(&mut self, idx: usize) {
        self._player_idx = idx;
    }

    pub fn increment_player_idx(&mut self) {
        self._player_idx += 1;
        self._player_idx %= self.config.num_players;
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
        // Appends actions to the top of the stack (it is a LIFO queue)
        // Note that hence first element of actions will become the last action to be taken this round
        self._action_queue.splice(..0, actions.iter_mut().map(|x| x.clone()));
    }

    pub fn push_callback(&mut self, callback: BirdCardCallback) {
        self._callbacks.push(callback)
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
    pub fn new(
        hand_limit: Option<u8>,
        num_players: Option<u8>,
    ) -> PyResult<Self> {
        let mut builder = &mut WingspanEnvConfigBuilder::create_empty();
        if let Some(hand_limit) = hand_limit {
            builder = builder.hand_limit(hand_limit);
        }
        if let Some(num_players) = num_players {
            builder = builder.num_players(num_players);
        }
        let config = builder.build().map_err(|err| PyValueError::new_err(format!("Error building config: {err}" )))?;

        Ok(Self {
            inner: WingspanEnv::try_new(config)
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

    pub fn action_space_size(slf: &Bound<'_, Self>) -> Option<usize> {
        slf.borrow().inner.action_space_size()
    }

    pub fn _debug_get_state(slf: &Bound<'_, Self>) -> (i8, usize, Option<String>, Vec<Player>) {
        let inner = &slf.borrow().inner;

        (inner._round_idx, inner._player_idx, inner._action_queue.last().map(|x| format!("{x:?}")), inner._players.clone())
    }

    pub fn next_action(slf: &Bound<'_, Self>) -> Option<PyAction> {
        slf.borrow().inner.next_action().map(PyAction::from)
    }
}
