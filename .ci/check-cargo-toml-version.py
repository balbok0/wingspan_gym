import tomllib
from pathlib import Path


CARGO_TOML_PATH = Path(__file__).parent.parent / "Cargo.toml"


def main():
    with open(CARGO_TOML_PATH, mode="rb") as f:
        cargo_toml = tomllib.load(f)
    print(cargo_toml["package"]["version"])


if __name__ == "__main__":
    main()
