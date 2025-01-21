use std::collections::VecDeque;

use derive_builder::Builder;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{action::Action, bird_card::{get_deck, BirdCard}, expansion::Expansion, player::Player};

#[derive(Debug, Builder, Clone)]
pub struct WingspanEnvConfig {
    #[builder(setter(into), default = 20)]
    hand_limit: u8,
    #[builder(setter(into), default = 2)]
    num_players: u8,
    #[builder(default = vec![])]
    expansions: Vec<Expansion>,
}


#[derive(Debug, Clone)]
pub struct WingspanEnv {
    config: WingspanEnvConfig,
    rng: StdRng,
    _round_idx: i8,
    _player_idx: u8,
    _bird_deck: Vec<BirdCard>,
    _players: Vec<Player>,
    _action_queue: VecDeque<Action>,
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
            _action_queue: VecDeque::with_capacity(50), // 50 seems like a reasonable upper bound even for most intense chains?
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

        // Give each player resources

        for _ in 0..self.config.num_players {
            let player_cards = self._bird_deck.split_off(self._bird_deck.len() - 5);
            self._players.push(Player::new(player_cards));
        }

        for _ in 0..5 {
            // Five times make current user decide on what to do
            self._action_queue.push_back(Action::DiscardFoodOrBirdCard);
        }

        // TODO: 3 birds face-up
    }

    pub fn step(&mut self, action: u8) {
        todo!("Not yet implemented")

        // TODO: Do the output here at least
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

    pub fn _debug_get_state(slf: &Bound<'_, Self>) -> (i8, u8, Vec<Player>) {
        let inner = &slf.borrow().inner;

        (inner._round_idx, inner._player_idx, inner._players.clone())
    }
}