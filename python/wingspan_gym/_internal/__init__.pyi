"""Internal package which contents are written in Rust.

Vast majority of this package is meant to provide introspection into under-the-hood state of the environment.
The main interactivity is provided with `step` and `reset` functions, although returns do not follow `gymnasium` spec.
"""

from enum import Enum
from typing import Optional, Union

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
    def actions(self) -> list[PyAction]: ...
    @property
    def players(self) -> list[Player]: ...
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

    def points(self) -> list[int]:
        """Number of points on per-player basis

        Returns:
            list[int]: List of current point tally for each player
        """
        ...

    def bird_deck(self) -> DeckAndHolder:
        """Bird Deck and Bird Card Holder.
        """
        ...

    def bird_feeder(self) -> BirdFeeder:
        """Bird Feeder (includes both dice in and out of it)
        """
        ...

    def callbacks(self) -> dict[int, set[BirdCardCallback]]:
        """Dictionary of all existing callbacks on per player basis.

        Callbacks correspond to Pink ("Once between turns...") powers, that are triggered based on other players actions.

        Returns:
            dict[int, set[BirdCardCallback]]: Mapping from player idx to
                list of callbacks this player has.
        """
        ...

    def active_callbacks(self) -> dict[int, set[BirdCardCallback]]:
        """Dictionary of currently active callbacks on per player basis.

        Namely, current players callbacks and all callbacks that already executed between turns of corresponding player are not included.
        Callbacks correspond to Pink ("Once between turns...") powers, that are triggered based on other players actions.

        Returns:
            dict[int, set[BirdCardCallback]]: Mapping from player idx to
                list of currently active callbacks this player has.
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

class Player:
    """
    Represents a player participating in a game of Wingspan.

    This class in read-only class that cannot be constructed or altered from python.
    It does not represented an agent, but rather an internal state of a player tracked by the wingspan environment.
    """

    @property
    def foods(self) -> bytes:
        """
        Food tokens that the player currently has.

        Returns:
            bytes: Bytes of length 5.
                Indexes correspond food types as defined in [FoodIndex][wingspan_gym._internal.FoodIndex].
        """
        ...

    @property
    def bird_cards(self) -> list[BirdCard]:
        """
        Bird cards currently held in hand of the player.

        Returns:
            list[BirdCard]: Bird cards in hand of the player.
        """
        ...

    @property
    def bonus_cards(self) -> list[BonusCard]:
        """
        Bonus cards currently held in hand of the player.

        Returns:
            list[BonusCard]: Bonus cards in hand of the player.
        """
        ...

    @property
    def turns_left(self) -> int:
        """
        Number of turns left by the current player in this round.
        If it is the current player, this turn is not included

        Returns:
            int: Number of turns left by the current player in this round.
        """
        ...

    @property
    def end_of_round_points(self) -> int:
        """
        Accumulated end of round points from end of round goals so far.

        These are added automatically by the environment engine.

        Returns:
            int: Accumulated end of round points from end of round goals so far.
        """
        ...

    def birds_on_mat(self) -> list[list[BirdCard]]:
        """
        Current birds on the player mat.

        Returns:
            list[list[BirdCard]]: List of lists of birds.
                Outer list is always of length 3 corresponding to habitats
                (in order of Forest, Grassland, Wetland).
                Inner list are birds placed in that habitat.
        """
        ...

class BirdCard:
    """
    Represents a bird card in Wingspan.

    Bird cards are static, and do not track whether the card has been placed,
    whether it has eggs, tucked cards etc.
    """

    @property
    def index(self) -> int:
        """Unique Index of the card.

        Since all of the cards are unique in Wingspan, this is also a unique id for instances of cards.

        They are sequentially increasing across indexes,
        but not necessarily if only some of the expansions are enabled.

        Returns:
            int: Index of the bird card.
        """
        ...

    @property
    def name(self) -> str:
        """Common Name of the bird.

        Returns:
            str: Common Name of the bird.
        """
        ...

    @property
    def cost(self) -> tuple[bytes, int, CostAlternative]:
        """Cost to play the card.

        Returns:
            tuple[bytes, int, CostAlternative]: Cost to play a card, represented as a tuple.
                Members of this tuples mean:
                    1. `bytes` - Bytes of length 5. Each byte represents cost of each [FoodIndex][wingspan_gym._internal.FoodIndex] to play the card.
                    2. `int` - Total number of food that one need to pay to play this card
                    3. `CostAlternative` - Whether cost is:
                        - Alternative (i.e. yes - For example "Fish/Seed")
                        - Cumulative (i.e. no - For example "Fish + Seed")
        """
        ...

    @property
    def color(self) -> BirdCardColor:
        """What is the color of bird card

        Returns:
            BirdCardColor: Color of the bird card.
        """
        ...

    @property
    def habitats(self) -> list[Habitat]:
        """
        What habitats does this belong to.

        Returns:
            list[Habitat]: Habitats this bird card can be played in.
        """
        ...

    @property
    def wingspan(self) -> int | None:
        """
        Wingspan size of the bird (in centimeters).

        Returns:
            int | None: Wingspan size of the bird (in centimeters).
                If the wingspan is None, it corresponds to "*" entry (meaning wingspan matches all of the wingspans).
        """
        ...

    @property
    def is_predator(self) -> bool:
        """Whether the card is a predator or not.

        Returns:
            bool: Whether the card has a predator ability or not.
        """
        ...

    @property
    def expansion(self) -> Expansion:
        """What expansion does this belong to.

        Returns:
            Expansion: Expansion this card was introduced in.
        """
        ...

    @property
    def bonus_card_membership(self) -> list[BonusCard]:
        """What bonus cards conditions this card bird card satisfies.

        Returns:
            list[BonusCard]: Bonus cards this card works for.
        """
        ...

class Habitat(Enum):
    """Enum representing 3 different habitats in Wingspan."""

    Forest = 0
    Grassland = 1
    Wetland = 2

BirdCardColor = Enum(
    "BirdCardColor",
    {
        "White": 0,
        "Brown": 1,
        "Pink": 2,
        "None": 3,
        "Teal": 4,
        "Yellow": 5,
    },
)

class CostAlternative(Enum):
    """Whether Bird Card cost is alternative resources (i.e. "/") or cumulative resources (i.e. "+")"""

    Yes = 0
    No = 1

class Expansion(Enum):
    """Enum representing different expansions in Wingspan.

    Note:
        Currently the only supported expansion is "Core".
    """

    Core = 0
    Asia = 1
    European = 2
    Oceania = 3

class BonusCard:
    """
    Represents a bonus card in Wingspan.

    Bonus cards are static, and do not track whether the card has in players possession etc.
    """

    @property
    def index(self) -> int:
        """Unique Index of the card.

        Since all of the cards are unique in Wingspan, this is also a unique id for instances of cards.

        They are sequentially increasing across indexes,
        but not necessarily if only some of the expansions are enabled.

        Returns:
            int: Index of the bonus card.
        """
        ...

    @property
    def name(self) -> str:
        """Name of the bonus cards.

        Returns:
            str: Title text of the bonus card.
        """
        ...

    @property
    def expansion(self) -> Expansion:
        """Which expansion this bonus card belongs to.

        Returns:
            Expansion: Expansion this card was introduced in.
        """
        ...

    @property
    def scoring_rule(self) -> ScoringRule:
        """How to score this card.

        Returns:
            ScoringRule: Scoring rule for this card.
                It is always a tuple with first element being `PyScoringRuleType`.
                For more details see `ScoringRule` documentation.
        """
        ...

class PyScoringRuleType:
    """
    Different types of scoring bonus cards based on number of birds satisfying condition.

    For more details see `ScoringRule` documentation.
    """

    Each = 0
    Ladder = 1

ScoringRule = Union[
    tuple[PyScoringRuleType.Each, int],
    tuple[PyScoringRuleType.Ladder, list[tuple[int, int]]],
]
ScoringRule.__doc__ = """ScoringRule represents a way to score a Bonus Card.

It is a tuple where first element is always `PyScoringRuleType` and second element is value applicable for that type.
There are two different types of scoring in Wingspan:

* Each - Where there is a fixed number of points for each of the birds satisfying the condition.
    In this case second value will be an integer.
    For example see [Rodentologist](https://navarog.github.io/wingsearch/card/1026).
* Ladder - Where there is a series of thresholds for number of birds satisfying a condition.
    After passing the threshold player gets fixed number of points.
    In this case second value is a list of increasing threshold tuples, where each tuple consists of:

        1. threshold value (number of birds needed to satisfy the condition)
        2. number of points if this threshold is satisfied.

    For example see [Cartographer](https://navarog.github.io/wingsearch/card/1007)
"""


class FoodIndex(Enum):
    """Enum representing different food types in the game of wingspan.

    These are actual food types existing, and not wild foods etc.
    """

    Invertebrate = 0
    Seed = 1
    Fish = 2
    Fruit = 3
    Rodent = 4


class DeckAndHolder:
    """Representation of Bird Card Deck and the Face Up Display.
    """

    @property
    def bird_deck(self) -> list[BirdCard]:
        """Actual deck of not yet used face-down cards.
        """
        ...

    @property
    def face_up_display(self) -> list[BirdCard]:
        """Face up display containing up to 3 bird cards
        """
        ...


class BirdFeeder:
    """Representation of dice in and out of Bird Feeder.
    """

    @property
    def dice_in_birdfeeder(self) -> list[int]:
        """Dice in bird feeder, that are ready to be taken out of it

        Returns:
            list[int]: List of integers representing dice faces.
                Values 0-4 correspond to those defined in [FoodIndex][wingspan_gym._internal.FoodIndex].
                Value 5 corresponds to Invertebrate/Seed face.
        """
        ...

    @property
    def dice_out_birdfeeder(self) -> list[int]:
        """Dice out of bird feeder, that are ready to be taken out of it

        Returns:
            list[int]: List of integers representing dice faces.
                Values 0-4 correspond to those defined in [FoodIndex][wingspan_gym._internal.FoodIndex].
                Value 5 corresponds to Invertebrate/Seed face.
        """
        ...


class BirdCardCallback:
    """Representation of information needed to trigger callback for Pink Powers Birds.

    Note that instead of being and actual callback (i.e. a function that takes arguments),
    it is list of arguments to said function.
    This function is triggered internally in Rust, on [BirdCard][wingspan_gym._internal.BirdCard] enum.

    Callbacks only ever exist for birds that are currently on the board.
    """

    @property
    def card(self) -> BirdCard:
        """Card this callback belongs to."""
        ...

    @property
    def habitat(self) -> Habitat:
        """Which habitat this card is currently in.
        """
        ...

    @property
    def card_idx(self) -> int:
        """Which bird number (left to right) this card is currently at.

        Note that bird number/index is different than column, since some birds can cover multiple columns.
        """
        ...

    @property
    def card_player_idx(self) -> int:
        """Which player (index) this card belongs to.
        """
        ...
