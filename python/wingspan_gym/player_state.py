
from . import constants, card_handling_utils
from .constants import BaseAction, ResourceArr
from .player_mat import PlayerMat

class PlayerState():
    def __init__(
        self,
        bird_cards: list[int],
        bonus_cards: list[int],
        resources: ResourceArr,
        *,
        player_mat: PlayerMat | None = None,
    ):
        self.bird_cards = bird_cards
        self.bonus_cards = bonus_cards
        self.resources = resources
        self.player_mat = player_mat or PlayerMat()

        # Optimization. Since we already check if cards can be played.
        # This allows us to determine that we can play cards directly
        self._next_playable_cards = []

    def discard_bird_card(self, card_idx: int) -> int | None:
        if card_idx >= len(self.bird_cards):
            return None

        return self.bird_cards.pop(card_idx)

    def discard_bonus_card(self, card_idx: int) -> int | None:
        if card_idx >= len(self.bonus_cards):
            return None

        return self.bonus_cards.pop(card_idx)

    def discard_resource(self, res_idx: int) -> int | None:
        if res_idx >= 5:
            return None
        if self.resources[res_idx] == 0:
            return None

        self.resources[res_idx] -= 1
        return res_idx

    def discard_resource_or_bird_card(self, idx: int) -> int | None:
        if idx < 5:
            return self.discard_resource(idx)
        else:
            return self.discard_bird_card(idx - 5)

    def perform_action(self, action_type: BaseAction) -> int | None:
        match action_type:
            case BaseAction.PLAY_A_BIRD:
                return self._can_play_birds()

    def _can_play_birds(self):
        if len(self.bird_cards) == 0:
            return None

        playable_cards = card_handling_utils.check_if_bird_cards_can_be_played(self.bird_cards, self.resources)

        if len(playable_cards) == 0:
            return None

        self._next_playable_cards = playable_cards

        return {
            "action_queue": [constants.NextAction.PLAY_A_CARD],
        }

    def _debug_print_state(self):
        print(f"\tBirds: {self.bird_cards}")
        print(f"\tBonus: {self.bonus_cards}")
        print(f"\tResources: {self.resources}")
        print(f"\t{constants.Resource.human_readable()}")
        pass
