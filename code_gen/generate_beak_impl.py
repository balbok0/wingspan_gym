from pathlib import Path

from utils import load_beak_json, common_name_to_enum_name

bird_beak_impl_file_path = (
    Path(__file__).parent.parent / "src" / "bird_card" / "bird_card_beak_impl.rs"
)


def main():
    beak_json = load_beak_json()

    with open(bird_beak_impl_file_path, mode="w") as f:
        # Imports
        f.writelines([
            "use super::BirdCard;\n",
            "use pyo3::prelude::*;\n",
            "\n",
        ])
        f.writelines([
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n",
            "#[pyclass(eq, eq_int)]\n",
            "pub enum BeakDirection {\n",
            "  Left,\n",
            "  Right,\n",
            "  None,\n",
            "}\n\n"
        ])

        f.write("impl BirdCard {\n")

        # Beak dir function
        f.writelines([
            "  pub fn beak_direction(&self) -> BeakDirection {\n",
            "    match self {\n",
        ])
        for bird_json in beak_json:
            enum_name = common_name_to_enum_name(bird_json["Common name"])
            beak_dir = "None"
            if bird_json["Beak Pointing Left"] == "X":
                beak_dir = "Left"
            elif bird_json["Beak Pointing Right"] == "X":
                beak_dir = "Right"
            f.write(f"      Self::{enum_name} => BeakDirection::{beak_dir},\n")
        f.writelines([
            "    }\n",
            "  }\n",
        ])

        # Close impl
        f.write("}\n")

    pass


if __name__ == "__main__":
    main()
