# Contributing

First of all: Thank You! If you see an issue, and decide to do something about it (either report it, or make a PR), it's great!

See sections below on more details

## Creating Issues

I'm using [GitHub Issues](https://github.com/balbok0/wingspan_gym/issues) to track problems. Please, search in roadmap and in open issues first before making a new one to avoid duplication. If you think your issue is unique, make one. There is no templates. This repo is meant to be tiny.

## Writing code

We use [uv](https://docs.astral.sh/uv/) and Rust (install via [rustup](https://rustup.rs/)).
Given that package is targeting python, I do not really care about Rust version, and I try to stay on the latest one (CI does!). This might change if it becomes a pain to maintain.
To start working on a package:

1. Run `uv sync` - This will install any packages. Typically needs to be only done once.
2. Then make changes to code you want
3. Run `uv run maturin develop` to built a package into current environment for testing
4. Repeat 2 + 3
5. Open a PR
