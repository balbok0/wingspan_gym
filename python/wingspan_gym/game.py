"""Main module containing Wingspan Environment."""

from typing import Optional

import gymnasium as gym
from ._internal import PyWingspanEnv


class WingspanEnv(gym.Env):
    def __init__(self):
        """gym Environment representing a game of Wingspan.

        It is single-threaded, but efficient offloading vast majority of operations to native implementation.
        """
        self._inner = PyWingspanEnv()

        # The biggest action space possible occurs when player needs to choose a card from their hand
        # or place an egg on a mat (15 choices max)
        # TODO: Make this into a valid check like the one commented below
        self.action_space = gym.spaces.Discrete(20)
        # self.action_space = gym.spaces.Discrete(max(15, self.config.hand_limit)))

    def reset(self, *, seed: Optional[int] = None):  # pyright: ignore[reportIncompatibleMethodOverride]
        """Resets environment to initial state.

        If specified, seed can be used for reproducibility.
        """
        assert seed is None or seed >= 0
        self._inner.reset(seed)

    def step(self, action: int):  # pyright: ignore[reportIncompatibleMethodOverride]
        return self._inner.step(action)

    def action_space_size(self) -> int:
        inner_result = self._inner.action_space_size()
        if inner_result is None:
            raise ValueError("Action space is non-existent in terminated state")
        return inner_result

    def cur_player(self) -> int:
        return self._inner.player_idx

    def cur_round(self) -> int:
        return self._inner.round_idx

    def _debug_print_state(self):
        round_idx, player_idx, action, players, callbacks = self._inner._debug_get_state()
        print(f"Current round: {round_idx}")
        print(f"Current player: {player_idx}")
        print(f"Next Action: {action}")
        print(f"Callbacks: {callbacks}")

        print("Cur player's hand:")
        self._print_player(players[player_idx])

    @staticmethod
    def _print_player(player):
        print(f"  Birds: {player.bird_cards}")
        # print(f"  Bonus: {self.bonus_cards}")
        print(f"  Foods: {player.foods}")

        placed_birds = player.birds_on_mat()
        print("  Mat:")
        for row_type, birds in zip(["F", "G", "W"], placed_birds):
            print(f"    {row_type}: {birds}")

    def next_action(self):
        return self._inner.next_action()
