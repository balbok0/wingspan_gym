use derive_builder::Builder;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{action::Action, bird_card::get_deck, bird_feeder::BirdFeeder, deck_and_holder::DeckAndHolder, error::{WingError, WingResult}, expansion::Expansion, habitat::Habitat, player::Player};

#[derive(Debug, Builder, Clone)]
pub struct WingspanEnvConfig {
    #[builder(setter(into), default = 20)]
    pub(crate) hand_limit: u8,
    #[builder(setter(into), default = 2)]
    pub(crate) num_players: usize,
    #[builder(default = vec![])]
    expansions: Vec<Expansion>,
}


#[derive(Debug, Clone)]
pub struct WingspanEnv {
    config: WingspanEnvConfig,
    pub(crate) rng: StdRng,
    _round_idx: i8,
    _player_idx: usize,
    pub(crate) _bird_deck: DeckAndHolder,
    _players: Vec<Player>,
    pub(crate) _bird_feeder: BirdFeeder,
    pub(crate) _action_queue: Vec<Action>,
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
            _bird_feeder: Default::default(),
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
        self._bird_deck = DeckAndHolder::new(deck);

        // TODO: Bonus cards for players

        // Give each player foods

        for _ in 0..self.config.num_players {
            let player_cards = self._bird_deck.draw_cards_from_deck(5);
            self._players.push(Player::new(player_cards));
        }

        self._action_queue.clear();
        for _ in 0..5 {
            // Five times make current user decide on what to do
            self._action_queue.push(Action::DiscardFoodOrBirdCard);
        }

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

        // TODO: End of round abilities
        // TODO: End of round goals

        // Start of the new round
        for player in self._players.iter_mut() {
            player.set_turns_left(8 - self._round_idx as u8);
        }
        self._bird_deck.reset_display();
    }

    pub fn step(&mut self, action_idx: u8) -> WingResult<()> {

        // unwrap is safe, since there is a check in the end
        let action = self._action_queue.last().unwrap().clone();
        if !action.is_performable(self) {
            println!("Action is not performable");
            return Err(WingError::InvalidAction);
        }
        let action = self._action_queue.pop().unwrap();
        if let Err(e) = action.perform_action(action_idx, self) {
            self._action_queue.push(action);
            return Err(e);
        };

        // Ensure that next action can be performed
        while !self._action_queue.is_empty() && !self._action_queue.last().unwrap().clone().is_performable(self) {
            self._action_queue.pop();
        }
        println!("After while check!");

        // Handle end of turn for the player
        if self._action_queue.is_empty() {
            println!("Queue is empty");
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
            } else if self._round_idx == 3 {
                // End of game is after Round 4 (0 - when it is zero indexed)
                todo!("End of game todo!")
            } else {
                self._player_idx %= self.config.num_players;
                // Normal rounds
                if self.current_player().turns_left == 0 {
                    // End of round
                    self.end_of_round();
                } else {
                    // End of normal turn
                    self.end_of_turn();
                }
                self._action_queue.push(Action::ChooseAction);

                // Reduce number of turns left, since a new player will be making a move
                self.current_player_mut().turns_left -= 1;
            }
        }

        println!("Queue size: {:?}\n", self._action_queue);
        Ok(())
    }

    pub fn populate_action_queue_from_habitat_action(&mut self, habitat: &Habitat) {
        let mut actions = self.current_player_mut().mat.get_actions(habitat);

        println!("Actions from queue: {actions:?}");

        self._action_queue.append(&mut actions);
        println!("Current queue: {:?}", self._action_queue);
    }

    pub fn current_player(&self) -> &Player {
        &self._players[self._player_idx]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self._players[self._player_idx]
    }

    pub fn num_choices(&self) -> Option<usize> {
        self._action_queue.last().map(|x| x.num_choices(&self))
    }

    pub fn config(&self) -> &WingspanEnvConfig {
        &self.config
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
            Ok(_x) => return Ok(None),
            Err(WingError::InvalidAction) => return Ok(None),
            Err(x) => return Err(x.into()),
        }
    }

    pub fn num_choices(slf: &Bound<'_, Self>) -> Option<usize> {
        slf.borrow().inner.num_choices()
    }

    pub fn _debug_get_state(slf: &Bound<'_, Self>) -> (i8, usize, Option<String>, Vec<Player>) {
        let inner = &slf.borrow().inner;

        (inner._round_idx, inner._player_idx, inner._action_queue.last().map(|x| format!("{x:?}")), inner._players.clone())
    }
}
