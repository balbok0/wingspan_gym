from queue import LifoQueue

import numpy as np
from .constants import NextAction


FOREST_WETLAND_ACTIONS_PER_BIRDS = [1, 1, 2, 2, 3, 3]
GRASSLAND_ACTIONS_PER_BIRDS = [2, 2, 3, 3, 4, 4]
ADDITION_ACTION_PER_BIRDS = {
    0: False,
    1: True,
    2: False,
    3: True,
    4: False,
    5: True,
}


class PlayerMat:
    def __init__(self):
        self.forest_birds = []
        self.forest_eggs = np.zeros(5, dtype=np.uint8)
        self.forest_egg_cap = np.zeros(5, dtype=np.uint8)
        self.grassland_birds = []
        self.grassland_eggs = np.zeros(5, dtype=np.uint8)
        self.grassland_egg_cap = np.zeros(5, dtype=np.uint8)
        self.wetland_birds = []
        self.wetland_eggs = np.zeros(5, dtype=np.uint8)
        self.wetland_egg_cap = np.zeros(5, dtype=np.uint8)

    def can_place_egg(self):
        return (
            np.any(self.forest_egg_cap > self.forest_eggs)
            or np.any(self.grassland_egg_cap > self.grassland_eggs)
            or np.any(self.wetland_egg_cap > self.wetland_eggs)
        )

    def forest_action(self):
        return self.general_action(
            FOREST_WETLAND_ACTIONS_PER_BIRDS,
            self.forest_birds,
            NextAction.GET_RESOURCE,
        )

    def grassland_action(self):
        return self.general_action(
            GRASSLAND_ACTIONS_PER_BIRDS,
            self.grassland_birds,
            NextAction.GET_EGG,
        )

    def wetland_action(self):
        return self.general_action(
            FOREST_WETLAND_ACTIONS_PER_BIRDS,
            self.wetland_birds,
            NextAction.GET_BIRD_CARD,
        )

    @staticmethod
    def general_action(action_num_list, birds, next_action: NextAction):
        action_queue = LifoQueue()

        # TODO: Get bird actions here

        # TODO: Optional things need to somehow clarify what action should be taken after


        for _ in range(action_num_list[len(birds)]):
            action_queue.put(next_action)

        return {
            "action_queue": action_queue
        }
