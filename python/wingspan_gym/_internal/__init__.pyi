from enum import Enum
from typing import Optional

class PyWingspanEnv:
    def __init__(
        self, hand_limit: Optional[int] = None, num_players: Optional[int] = None
    ):
        """
        Initializes environment with specified number of players.
        Additionally it enforces hand limit, which in turn can limits size of action space.

        Args:
            hand_limit (Optional[int], optional): Maximum size of hand (bird cards only) allowed for player to have. Defaults to 20.
            num_players (Optional[int], optional): Number of players in a game. Note, that going way above the regular max number of players (5), might cause instability.
                Defaults to 2.
        """
        ...

    @property
    def player_idx(self) -> int:
        """Index of a current player."""
        ...

    @property
    def round_idx(self) -> int:
        """
        Index of a current round played.

        It is 0-indexed, with 3 as a max value. -1 indicates setup phase, when player chooses which resources/cards to discard.
        """

    def reset(self, seed: Optional[int]):
        """Resets the environment bringing it to the beginning of setup phase.

        Args:
            seed (Optional[int]): Random seed to use when setting up a game.
        """
        ...

    def step(self, action_idx: int) -> StepResult:
        """Performs a step for a current player.

        Args:
            action_idx (int): Index of action to take.
                This is highly game state dependent, and implementation/guide of the various possible contexts is in the works.

        Returns:
            StepResult: StepResult indicating whether action was successful or not, and basic feedback about game state.
        """
        ...

    def action_space_size(self) -> Optional[int]:
        """Current allowed action space size.

        Not all of the actions are necessarily allowed, even if they lie within the action space.

        Returns:
            Optional[int]: Size of action space. None, if game is in terminated state.
        """
        ...

    def _debug_get_state(self):
        """Highly variable function used for debugging.

        Returns a lot of information about game, but not in an efficient manner.

        **Unless you are actively developing code in wingspan env repo, avoid using it**
        """
        ...

    def next_action(self) -> Optional[PyAction]:
        """
        Returns what the next action type is.

        Returns:
            Optional[PyAction]: Next action to be performed. None, if game is in terminated state.
        """
        ...

class StepResult(Enum):
    """Internal enum describing result of the action."""

    Live = 0
    Terminated = 1
    Invalid = 2

class PyAction:
    """A type of action that can be performed."""

    def __str__(self) -> str:
        """String representation of this PyAction."""
        ...
