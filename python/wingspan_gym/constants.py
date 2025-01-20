from enum import Enum
from typing import Optional

from numpy.typing import NDArray
from jaxtyping import UInt8


class NextAction(Enum):
    CHOOSE_ACTION = 0

    # Section: Discards
    DISCARD_BIRD_CARD_OR_RESOURCE = 1
    DISCARD_BONUS_CARD = 2
    DISCARD_BIRD_CARD = 3
    DISCARD_RESOURCE = 4

    # Section: Turn Actions
    PLAY_A_CARD = 5

    # Section: Draw/Get resources
    GET_BIRD_CARD = 6
    GET_BONUS_CARD = 7
    GET_RESOURCE = 8
    GET_EGG = 9

    # Section: Optional action decision
    OPTIONAL_ACTION = 100

    NOT_IMPLEMENTED_YET = 999



class BaseAction(Enum):
    PLAY_A_BIRD = 0
    FOREST = 1
    GRASSLAND = 2
    WETLAND = 3

    def try_new(idx: int) -> Optional["BaseAction"]:
        if idx > 3:
            return None

        return BaseAction(idx)

class Resource(Enum):
    INVERTEBRATE = 0
    SEED = 1
    FISH = 2
    FRUIT = 3
    RODENT = 4

    # FIXME: Not currently used
    NECTAR = 5

    @staticmethod
    def regular_resources():
        return [
            Resource.INVERTEBRATE,
            Resource.SEED,
            Resource.FISH,
            Resource.FRUIT,
            Resource.RODENT
        ]

    @staticmethod
    def column_names():
        return [
            "Invertebrate",
            "Seed",
            "Fish",
            "Fruit",
            "Rodent",
        ]

    def human_readable():
        return ["I", "S", "Fi", "Fr", "R"]

ResourceArr = UInt8[NDArray, "5"]
