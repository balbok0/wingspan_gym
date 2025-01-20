import dataclasses
from pathlib import Path

import gymnasium as gym
import numpy as np
import polars as pl

from . import constants
from .constants import NextAction, Resource
from .player_state import PlayerState
from .card_handling_utils import load_cards


@dataclasses.dataclass
class WingspanEnvConfig:
    hand_limit: int = 20
    num_players: int = 2

    # FIXME: No extensions are supported for now, since they do change game mechanics quite a bit.
    extensions = []

    def __post_init__(self):
        # TODO: Might need to tune this number. 4 actions, 5 resources
        if self.hand_limit < 5:
            raise ValueError("Invalid hand limit, it needs to be at least 4 cards")
        if self.num_players < 2 or self.num_players > 5:
            raise ValueError("Invalid number of players. Needs to be 2-5 players.")


class WingspanEnv(gym.Env):
    def __init__(
        self,
        *,
        config: WingspanEnvConfig | None = None
    ):
        self.config = config or WingspanEnvConfig()

        # The biggest action space possible occurs when player needs to choose a card from their hand
        # or place an egg on a mat (15 choices max)
        self.action_space = gym.spaces.Discrete(max(15, self.config.hand_limit))

        # TODO: Initialize resources etc.
        pass

    def reset(self, *, seed: int = None, options = None):
        self._rng = np.random.default_rng(seed=seed)
        self._round_idx = -1  # -1 means it's still setup
        self._player_idx = 0

        # Load all of the cards
        # TODO: This will need to be reworked probably
        self._birds, self._bonus, self._all_goals = load_cards()
        self._birds = self._birds["index"].to_numpy().copy()
        self._bonus = self._bonus["index"].to_numpy().copy()
        self._rng.shuffle(self._birds)
        self._rng.shuffle(self._bonus)

        # For each player give them one of each resource
        num_resources = len(Resource.regular_resources())
        self.resources = {
            player_idx: np.ones(num_resources, dtype=np.int8)
            for player_idx in range(self.config.num_players)
        }

        # Initialize players
        players = []
        for player_idx in range(self.config.num_players):
            players.append(PlayerState(
                bird_cards=self._birds[player_idx * 5:(player_idx + 1) * 5].tolist(),
                bonus_cards=self._bonus[player_idx * 2:(player_idx + 1) * 2].tolist(),
                resources=np.ones(num_resources, dtype=np.uint8),
            ))
        self.players: list[PlayerState] = players

        # Select round goals
        self._next_action = NextAction.DISCARD_BIRD_CARD_OR_RESOURCE
        self._discard_remaining = 5

        print(f"birds: {len(self._birds)}")
        print(f"bonus: {len(self._bonus)}")
        print()

        return super().reset(seed=seed, options=options)

    def step(self, action):

        result = None
        cur_player = self.players[self._player_idx]
        match self._next_action:
            case NextAction.DISCARD_BIRD_CARD_OR_RESOURCE:
                result = cur_player.discard_resource_or_bird_card(action)
            case NextAction.DISCARD_BIRD_CARD:
                result = cur_player.discard_bird_card(action)
            case NextAction.DISCARD_BONUS_CARD:
                result = cur_player.discard_bonus_card(action)
            case NextAction.CHOOSE_ACTION:
                base_action = constants.BaseAction.try_new(action)
                if base_action is not None:
                    result = cur_player.perform_action(base_action)
                else:
                    result = None

        if result is None:
            # TODO: Indicate that action was illegal smh
            return (None, None, None, None, None)

        # Action was legal. Update state accordingly
        if self._round_idx == -1:
            # Setup phase
            self._discard_remaining -= 1

            if self._discard_remaining == 0:
                if self._next_action == NextAction.DISCARD_BIRD_CARD_OR_RESOURCE:
                    self._discard_remaining = 1
                    self._next_action = NextAction.DISCARD_BONUS_CARD
                elif self._player_idx == self.config.num_players - 1:
                    self._round_idx = 0
                    self._player_idx = 0
                    self._next_action = NextAction.CHOOSE_ACTION
                else:
                    self._discard_remaining = 5
                    self._next_action = NextAction.DISCARD_BIRD_CARD_OR_RESOURCE
                    self._player_idx += 1

            # Early return. Get some state encoding, based on the fact that game hasn't started yet
            return (None, None, None, None, None)



        observation = None
        reward = None
        terminated = None
        truncated = None
        info = None
        return (None, None, None, None, None)
        return super().step(action)

    def _post_setup_update(self):
        """
        This function performs post-setup (i.e. players choosing their cards) actions.
        """

        self.round_goals = self._rng.choice(self._all_goals["index"], 4)

        pass

    def _debug_print_state(self):
        print(f"Current round: {self._round_idx}")
        print(f"Current player: {self._player_idx}")

        print("Cur player's hand:")
        self.players[self._player_idx]._debug_print_state()
