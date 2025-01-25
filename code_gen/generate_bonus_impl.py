from pathlib import Path

import polars as pl

from utils import load_all_cards, FOOD_TYPES, HABITATS

bonus_impl_file_path = Path(__file__).parent.parent / "src" / "bonus_card" / "bonus_card_impl.rs"


def main():
    birds, bonuses, goals = load_all_cards()

    with open(bonus_impl_file_path, mode="w") as f:
        # Imports
        f.writelines([
            "// This code is generated automatically via a script in code_gen/ folder\n",
            "use strum_macros::EnumIter;\n\n",
            "use super::ScoringRule;\n",
            # "use crate::{{expansion::Expansion, food::{{BirdCardCost, CostAlternative}}, nest::NestType}};\n",
        ])

        # Start with enum
        f.writelines([
            "\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]",
            "\npub enum BonusCard {\n"
        ])

        for bonus in bonuses.iter_rows(named=True):
            enum_name = bonus["enum_name"]
            f.write(f"  {enum_name},\n")
        f.write("}\n")

        # Impl block
        f.writelines([
            "\n",
            "impl BonusCard {\n",
        ])

        # Index function
        f.writelines([
            "  pub fn index(&self) -> u16 {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => {row['index']},\n"
            for row in bonuses.iter_rows(named=True)
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
            f"      Self::{row['enum_name']} => \"{row['Bonus card']}\",\n"
            for row in bonuses.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Scoring rule function
        f.writelines([
            "\n",
            "  pub fn scoring_rule(&self) -> ScoringRule {\n",
            "    match self {\n",
        ])
        f.writelines([
            f"      Self::{row['enum_name']} => {row['points_expr']},\n"
            for row in bonuses.iter_rows(named=True)
        ])
        f.writelines([
            "    }\n",
            "  }\n"
        ])

        # Close impl block
        f.write("}\n")


if __name__ == "__main__":
    main()
