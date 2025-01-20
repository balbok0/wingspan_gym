import numpy as np
import pytest

import wingspan_gym.card_handling_utils as chu

@pytest.mark.parametrize(
    ("bird_indexes", "resources", "expected"),
    (
        (  # OR resources
            [10],  # American Robin
            np.array([1, 0, 0, 1, 0]),
            [10]
        ),
        (  # AND resources
            [9],  # American Redstart
            np.array([1, 0, 0, 1, 0]),
            [9]
        ),
        (  # Both of the above
            [9, 10],
            np.array([1, 0, 0, 1, 0]),
            [9, 10]
        ),
        (  # Both of the above - None matches reqs
            [9, 10],
            np.array([0, 0, 0, 1, 0]),
            []
        ),
        (  # Only one matches
            [6, 10],
            np.array([0, 2, 0, 1, 0]),
            [6]
        ),
        (  # Wild food - enough resources
            [4],
            np.array([0, 1, 0, 1, 0]),
            [4]
        ),
        (  # Wild food - enough resources (but it's the same resource)
            [4],
            np.array([0, 2, 0, 0, 0]),
            [4]
        ),
        (  # Wild food - missing resources for wild food
            [4],
            np.array([0, 1, 0, 0, 0]),
            []
        )
    )
)
def test_check_if_bird_cards_can_be_played(bird_indexes, resources, expected):
    actual = chu.check_if_bird_cards_can_be_played(bird_indexes, resources)

    np.testing.assert_array_equal(
        actual,
        expected
    )
