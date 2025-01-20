from pathlib import Path
from functools import reduce

import polars as pl

from .constants import Resource, ResourceArr
from . import _internal


def __load_all_cards():
    spread_sheet_path = Path(__file__).parent.parent.parent / "data/wingspan-20221201.xlsx"
    birds = pl.read_excel(spread_sheet_path, sheet_name="Birds").with_row_index()
    bonus = pl.read_excel(spread_sheet_path, sheet_name="Bonus").with_row_index()
    goals = pl.read_excel(spread_sheet_path, sheet_name="Goals").with_row_index()

    # Modify birds so that columns are nicer
    into_int_expr = lambda col: pl.col(col).cast(pl.Int8, strict=False)
    birds = birds.with_columns(
        *[into_int_expr(col) for col in Resource.column_names()],
        (pl.col("/ (food cost)") == "x").fill_null(False).alias("food_cost_alt"),
        into_int_expr("Total food cost").alias("Total"),
        *[(pl.col(col) == "X").fill_null(False).alias(col) for col in ["Forest", "Grassland", "Wetland"]],
    )

    return birds, bonus, goals


ALL_BIRD_CARDS, ALL_BONUS_CARDS, ALL_GOALS = __load_all_cards()


def load_cards(extensions: list[str] | None = None):
    extensions = extensions or ["core"]
    if "core" not in extensions:
        extensions.append("core")
    extensions = pl.lit(extensions)
    is_in_extensions = pl.col("Set").str.split(", ").list.set_intersection(extensions).list.len() > 0

    birds = ALL_BIRD_CARDS.filter(is_in_extensions)
    bonus = ALL_BONUS_CARDS.filter(is_in_extensions)
    goals = ALL_GOALS.filter(is_in_extensions)

    return birds, bonus, goals


def check_if_bird_cards_can_be_played(
    bird_card_idxs: list[int],
    resources: ResourceArr
) -> list[int]:

    selected_cards = ALL_BIRD_CARDS[bird_card_idxs]
    resource_total = resources.sum()

    is_playable_resource = lambda res_idx, res_name: (
        pl.when(
            pl.col(res_name).is_null()
        ).then(
            # If it is null make sure column does not affect the outcome
            # True when cost is not an alternative (since it's an AND)
            # False when cost is an alternative (since it's an OR)
            ~pl.col("food_cost_alt")
        ).otherwise(
            pl.col(res_name) <= resources[res_idx]
        )
    )

    is_playable_card = (
        pl.when(pl.col("food_cost_alt"))
        .then(
            # NOTE: No birds have OR and wild food (i.e. Any food item), thus no need to do additional check
            pl.fold(
                False,
                pl.Expr.or_,
                [
                    is_playable_resource(res_idx, res_name)
                    for res_idx, res_name in enumerate(Resource.column_names())
                ]
            )
        )
        .otherwise(
            pl.fold(
                True,
                pl.Expr.and_,
                [
                    is_playable_resource(res_idx, res_name)
                    for res_idx, res_name in enumerate(Resource.column_names())
                ]
            ) & (pl.col("Total") <= resource_total)
        )
    )

    selected_cards = selected_cards.filter(is_playable_card)
    print(ALL_BIRD_CARDS[bird_card_idxs]["Common name"])
    print(resources)
    print(Resource.human_readable())
    print(selected_cards)
    # exit(0)

    return selected_cards["index"].to_list()
