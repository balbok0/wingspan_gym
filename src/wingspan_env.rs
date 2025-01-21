use derive_builder::Builder;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{action::Action, bird_card::{get_deck, BirdCard}, error::{WingError, WingResult}, expansion::Expansion, player::Player};

#[derive(Debug, Builder, Clone)]
pub struct WingspanEnvConfig {
    #[builder(setter(into), default = 20)]
    hand_limit: u8,
    #[builder(setter(into), default = 2)]
    num_players: usize,
    #[builder(default = vec![])]
    expansions: Vec<Expansion>,
}


#[derive(Debug, Clone)]
pub struct WingspanEnv {
    config: WingspanEnvConfig,
    rng: StdRng,
    _round_idx: i8,
    _player_idx: usize,
    _bird_deck: Vec<BirdCard>,
    _players: Vec<Player>,
    _action_queue: Vec<Action>,
}

impl WingspanEnv {
    pub fn try_new(config: WingspanEnvConfig) -> Self {
        let num_players = config.num_players;
        let mut env = WingspanEnv {
            config,
            rng: StdRng::from_entropy(),
            _round_idx: -1,
            _player_idx: 0,
            _bird_deck: Vec::new(),
            _players: Vec::with_capacity(num_players as usize),
            _action_queue: Vec::with_capacity(50), // 50 seems like a reasonable upper bound even for most intense chains?
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
        let mut deck = get_deck(&self.config.expansions);
        deck.shuffle(&mut self.rng);
        self._bird_deck = deck;

        // TODO: Bonus cards for players

        // Give each player foods

        for _ in 0..self.config.num_players {
            let player_cards = self._bird_deck.split_off(self._bird_deck.len() - 5);
            self._players.push(Player::new(player_cards));
        }

        self._action_queue.clear();
        for _ in 0..5 {
            // Five times make current user decide on what to do
            self._action_queue.push(Action::DiscardFoodOrBirdCard);
        }

        // TODO: 3 birds face-up
    }

    pub fn step(&mut self, action_idx: u8) -> WingResult<()> {

        // unwrap is safe, since there is a check in the end
        let action = self._action_queue.last().unwrap().clone();

        action.perform_action(action_idx, self)?;
        self._action_queue.pop();

        // Handle end of turn for the player
        if self._action_queue.is_empty() {
            // Loop through players
            self._player_idx += 1;

            // Special case is first round
            if self._round_idx == -1 {
                if self._player_idx == self.config.num_players {
                    // Setup is done from players side.
                    // TODO: Finish it here and then make it round 0
                    self._round_idx = 0;
                    self._player_idx = 0;
                    for player in self._players.iter_mut() {
                        player.set_turns_left(8);
                    }
                    self._action_queue.push(Action::ChooseAction);
                    // Reduce number of turns left, since a new player will be making a move
                    self.current_player_mut().turns_left -= 1;
                } else {
                    // Next player can do setup
                    for _ in 0..5 {
                        // Five times make current user decide on what to do
                        self._action_queue.push(Action::DiscardFoodOrBirdCard);
                    }
                }
            } else {
                if self.current_player().turns_left == 0 {
                    // Start of the new round
                    for player in self._players.iter_mut() {
                        player.set_turns_left(8 - self._round_idx as u8);
                    }
                    todo!("New round logic not yet implemented");
                }
                self._action_queue.push(Action::ChooseAction);

                // Reduce number of turns left, since a new player will be making a move
                self.current_player_mut().turns_left -= 1;
            }

        }

        Ok(())
    }

    pub fn current_player(&self) -> &Player {
        &self._players[self._player_idx]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self._players[self._player_idx]
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

    #[pyo3(signature = (seed=None))]
    pub fn reset(slf: &Bound<'_, Self>, seed: Option<u64>) {
        slf.borrow_mut().inner.reset(seed)
    }

    pub fn step(slf: &Bound<'_, Self>, action_idx: u8) -> PyResult<Option<()>> {
        match slf.borrow_mut().inner.step(action_idx) {
            // Ok(x) => return Ok(Some(x)),
            // FIXME: for now returning none, so it doesn't freak out
            Ok(x) => return Ok(None),
            Err(WingError::InvalidAction) => return Ok(None),
            Err(x) => return Err(x.into()),
        }
    }

    pub fn _debug_get_state(slf: &Bound<'_, Self>) -> (i8, usize, Vec<Player>) {
        let inner = &slf.borrow().inner;

        (inner._round_idx, inner._player_idx, inner._players.clone())
    }
}