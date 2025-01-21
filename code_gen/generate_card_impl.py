from pathlib import Path

import polars as pl
import unidecode

bird_impl_file_path = Path(__file__).parent.parent / "src" / "bird_card" / "bird_card_impl.rs"


FOOD_TYPES = ["Invertebrate", "Seed", "Fish", "Fruit", "Rodent"]
HABITATS = ["Forest", "Grassland", "Wetland"]


def __load_all_cards():
    spread_sheet_path = Path(__file__).parent.parent / "data/wingspan-20221201.xlsx"

    birds = pl.read_excel(spread_sheet_path, sheet_name="Birds").with_row_index()
    bonus = pl.read_excel(spread_sheet_path, sheet_name="Bonus").with_row_index()
    goals = pl.read_excel(spread_sheet_path, sheet_name="Goals").with_row_index()

    # For now, just core package
    is_in_extensions = pl.col("Set").str.split(", ").list.set_intersection(["core"]).list.len() > 0
    birds = birds.filter(is_in_extensions)
    bonus = bonus.filter(is_in_extensions)
    goals = goals.filter(is_in_extensions)

    # Modify birds so that columns are nicer
    into_int_expr = lambda col: pl.col(col).cast(pl.Int8, strict=False)
    birds = birds.with_columns(
        *[into_int_expr(col) for col in FOOD_TYPES],
        (pl.col("/ (food cost)") == "/").fill_null(False).alias("food_cost_alt"),
        into_int_expr("Total food cost").alias("Total"),
        *[(pl.col(col) == "X").fill_null(False).alias(col) for col in ["Forest", "Grassland", "Wetland"]],
        pl.col("Color").fill_null("None").alias("Color"),
        into_int_expr("Victory points").fill_null(0).alias("Victory points"),
        pl.col("Nest type").fill_null("None").alias("Nest type"),
        (pl.col("Predator") == "X").fill_null(False).alias("is_predator"),
    )

    return birds, bonus, goals


def common_name_to_enum_name(name: str) -> str:
    return unidecode.unidecode(name.strip().replace(" ", "").replace("'", "").replace("-", ""), )



def main():
    birds, bonus, goals = __load_all_cards()

    enum_names = []

    with open(bird_impl_file_path, mode="w") as f:
        # Imports
        f.writelines([
            "use strum_macros::EnumIter;\n\n",
            "use super::BirdCardColor;\n",
            "use crate::{{habitat::Habitat, nest::NestType, resource::{{BirdCardCost, CostAlternative}}}};\n",
        ])

        # Start with enum
        f.writelines([
            "\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]",
            "\npub enum BirdCard {\n"
        ])

        for bird in birds.iter_rows(named=True):
            enum_name = common_name_to_enum_name(bird["Common name"])
            f.write(f"  {enum_name},\n")

            enum_names.append(enum_name)
        f.write("}\n")

        birds.insert_column(0, column=pl.Series("enum_name", enum_names))

        # Impl block
        f.writelines([
            "\n",
            "impl BirdCard {\n",
        ])

        # Index function
        f.writelines([
            "  pub fn index(&self) -> u16 {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => {row['index']},\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Name function
        f.writelines([
            "\n",
            "  pub fn name(&self) -> &'static str {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => \"{row['Common name']}\",\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Cost function
        f.writelines([
            "\n",
            "  pub fn cost(&self) -> &'static BirdCardCost {\n",
            "    match self {\n",
        ])
        cost_lines = []
        for row in birds.iter_rows(named=True):
            cost_line = f"      Self::{row['enum_name']} => &(["
            for food_type in FOOD_TYPES:
                rust_food_type = "None" if row[food_type] is None else f"Some({row[food_type]})"
                cost_line += f"{rust_food_type}, "
            cost_line = cost_line[:-2]
            cost_line += f"], {row['Total']}, CostAlternative::"
            cost_line += "Yes" if row["food_cost_alt"] else "No"
            cost_line += "),\n"
            cost_lines.append(cost_line)
        f.writelines(cost_lines)
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Color function
        f.writelines([
            "\n",
            "  pub fn color(&self) -> &BirdCardColor {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => &BirdCardColor::{row['Color'].capitalize()},\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Victory points function
        f.writelines([
            "\n",
            "  pub fn points(&self) -> u8 {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => {row['Victory points']},\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Habitats function
        f.writelines([
            "\n",
            "  pub fn habitats(&self) -> &'static [Habitat] {\n",
            "    match self {\n",
        ])
        habitat_lines = []
        for row in birds.iter_rows(named=True):
            habitat_line = f"      Self::{row['enum_name']} => &["
            for habitat in HABITATS:
                if row[habitat]:
                    habitat_line += f"Habitat::{habitat}, "
            habitat_line = habitat_line[:-2]
            habitat_line += "],\n"
            habitat_lines.append(habitat_line)
        f.writelines(habitat_lines)
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Wingspan function
        f.writelines([
            "\n",
            "  pub fn wingspan(&self) -> Option<u16> {\n",
            "    match self {\n",
        ])
        wingspan_lines = []
        for row in birds.iter_rows(named=True):
            # NOTE: Flightless birds are "*"
            wingspan_val = (
                "None"
                if row["Wingspan"] == "*" else
                f"Some({row['Wingspan']})"
            )
            wingspan_lines.append(
                f"      Self::{row['enum_name']} => {wingspan_val},\n"
            )
        f.writelines(wingspan_lines)
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Egg Capacity function
        f.writelines([
            "\n",
            "  pub fn egg_capacity(&self) -> u8 {\n",
            "    match self {\n",
        ])
        wingspan_lines = []
        f.writelines([
            f"      Self::{row['enum_name']} => {row['Egg limit']},\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Nest Type function
        f.writelines([
            "\n",
            "  pub fn nest_type(&self) -> &NestType {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => &NestType::{row['Nest type'].capitalize()},\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # is_predator
        f.writelines([
            "\n",
            "  pub fn is_predator(&self) -> bool {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => {str(row['is_predator']).lower()},\n"
            for row in birds.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Close impl block
        f.write("}\n")





if __name__ == "__main__":
    main()
