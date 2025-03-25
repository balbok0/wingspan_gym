import json
from pathlib import Path
from typing import Any

import polars as pl
import unidecode


FOOD_TYPES = ["Invertebrate", "Seed", "Fish", "Fruit", "Rodent"]
HABITATS = ["Forest", "Grassland", "Wetland"]


def load_beak_json() -> list[dict[str, Any]]:
    json_data = Path(__file__).parent.parent / "data/master.json"
    with open(json_data, mode="rb") as f:
        return json.load(f)


def load_all_cards():
    spread_sheet_path = Path(__file__).parent.parent / "data/wingspan-20221201.xlsx"

    birds = pl.read_excel(spread_sheet_path, sheet_name="Birds").with_row_index()
    bonus = pl.read_excel(spread_sheet_path, sheet_name="Bonus").with_row_index()
    goals = pl.read_excel(spread_sheet_path, sheet_name="Goals").with_row_index()

    bonus = bonus.filter(~pl.col("Bonus card").str.starts_with("[automa]"))

    # For now, just core package
    set_col = pl.col("Set").str.split(", ").list.first().alias("expansion")
    # is_in_extensions = set_col.list.set_intersection(["core"]).list.len() > 0
    birds = birds.with_columns(set_col)  # .filter(is_in_extensions)
    bonus = bonus.with_columns(set_col)  # .filter(is_in_extensions)
    goals = goals.with_columns(set_col)  # .filter(is_in_extensions)

    # Modify birds so that columns are nicer
    def into_int_expr(col: str) -> pl.Expr:
        return pl.col(col).cast(pl.Int8, strict=False)

    birds = birds.with_columns(
        *[into_int_expr(col) for col in FOOD_TYPES],
        (pl.col("/ (food cost)") == "/").fill_null(False).alias("food_cost_alt"),
        into_int_expr("Total food cost").alias("Total"),
        *[
            (pl.col(col) == "X").fill_null(False).alias(col)
            for col in ["Forest", "Grassland", "Wetland"]
        ],
        pl.col("Color").fill_null("None").alias("Color"),
        into_int_expr("Victory points").fill_null(0).alias("Victory points"),
        pl.col("Nest type").fill_null("None").alias("Nest type"),
        (pl.col("Predator") == "X").fill_null(False).alias("is_predator"),
        pl.col("Common name")
        .map_elements(common_name_to_enum_name, return_dtype=pl.String)
        .alias("enum_name"),
    )

    bonus = bonus.with_columns(
        pl.col("Bonus card")
        .map_elements(common_name_to_enum_name, return_dtype=pl.String)
        .alias("enum_name"),
        pl.col("VP")
        .map_elements(bonus_vp_to_scoring, return_dtype=pl.String)
        .alias("points_expr"),
    )

    return birds, bonus, goals


def common_name_to_enum_name(name: str) -> str:
    return unidecode.unidecode(
        name.strip()
        .replace("[swift_start_asia]", "")
        .replace(" ", "")
        .replace("'", "")
        .replace("-", ""),
    )


def _scoring_rule_to_lb_points(scoring_rule: str) -> tuple[int, int]:
    scoring_rule = scoring_rule.strip()

    if scoring_rule.lower().startswith("one"):
        lb = 1
    elif scoring_rule.lower().startswith("two"):
        lb = 2
    else:
        lb = int(scoring_rule[0])

    points = scoring_rule.rsplit(":")[-1].strip()

    if points == "6/7":
        points = 7

    # Lower bound is always the first number
    return lb, int(points)


def bonus_vp_to_scoring(vp_name: str) -> str:
    vp_name = vp_name.strip()

    if ";" in vp_name:
        # It's in a form of:
        # A to B TYPE: P1; ...; C+ TYPE: P2
        scoring_rules = vp_name.split(";")

        lower_bound_and_points = []
        for scoring_rule in scoring_rules:
            lower_bound_and_points.append(_scoring_rule_to_lb_points(scoring_rule))

        return f"ScoringRule::Ladder(Box::new({lower_bound_and_points}))"
    elif "per" in vp_name or "each" in vp_name:
        point_each = int(vp_name[0])

        return f"ScoringRule::Each({point_each})"
    else:
        raise ValueError(f"Unrecognized Value Point rule {vp_name}")
