"""Main module containing Wingspan Environment."""

from typing import Any, Optional

import gymnasium as gym
from ._internal import Player, PyAction, PyWingspanEnv, StepResult

import numpy as np


class WingspanEnv(gym.Env[PyWingspanEnv, np.uint8]):
    def __init__(self):
        """gym Environment representing a game of Wingspan.

        It is single-threaded, but efficient offloading vast majority of operations to native implementation.
        """
        self._inner = PyWingspanEnv()

        # The biggest action space possible occurs when player needs to choose a card from their hand
        # or place an egg on a mat (15 choices max)
        # TODO: Make this into a valid check like the one commented below
        self.action_space = gym.spaces.Discrete(20)  # type: ignore
        # self.action_space = gym.spaces.Discrete(max(15, self.config.hand_limit)))

    def reset(self, *, seed: Optional[int] = None):  # pyright: ignore[reportIncompatibleMethodOverride]
        """Resets environment to initial state.

        If specified, seed can be used for reproducibility.
        """
        assert seed is None or seed >= 0
        self._inner.reset(seed)

    def step(self, action: np.uint8) -> tuple[PyWingspanEnv, float, bool, bool, dict[str, Any]]:
        result = self._inner.step(action)

        return self._inner, 0.0, result == StepResult.Terminated, False, {}

    def action_space_size(self) -> int:
        inner_result = self._inner.action_space_size()
        if inner_result is None:
            raise ValueError("Action space is non-existent in terminated state")
        return inner_result

    def cur_player_idx(self) -> int:
        """Returns index of current player

        Returns:
            int: Index of current player
        """
        return self._inner.player_idx

    def cur_round(self) -> int:
        """Returns index of current round.

        Round index can be -1, which means that game is still in setup phase (i.e. players are choosing which resource to discard).
        It is 0-indexed, so first round is 0.
        If it is equal to number of round in the game, this means that game is finished and in terminated state.

        Returns:
            int: Index of current round [-1, num_rounds]
        """
        return self._inner.round_idx

    def points(self) -> list[int]:
        """Current point tally for each of the players.

        This is calculated as if the game finished right now (although end of game/end of round abilities are not triggered).

        Returns:
            list[int]: Current point tally for each of the players.
        """
        return self._inner.points()

    def _debug_print_state(self):
        round_idx, player_idx, action, players, callbacks = (
            self._inner._debug_get_state()
        )
        print(f"Current round: {round_idx}")
        print(f"Current player: {player_idx}")
        print(f"Next Action: {action}")
        print(f"Callbacks: {callbacks}")

        print("Cur player's hand:")
        self._print_player(players[player_idx])

    @staticmethod
    def _print_player(player: Player):
        print(f"  Birds: {player.bird_cards}")
        print(f"  Bonus: {player.bonus_cards}")
        print(f"  Foods: {player.foods}")

        placed_birds = player.birds_on_mat()
        print("  Mat:")
        for row_type, birds in zip(["F", "G", "W"], placed_birds):
            print(f"    {row_type}: {birds}")

    def next_action(self) -> Optional[PyAction]:
        """What is the next action to be taken by current player

        Returns:
            Optional[PyAction]: Next action to be performed. None, if game is in terminated state.
        """
        return self._inner.next_action()
