# Wingspan Gym
Implementation of [board game Wingspan](https://stonemaiergames.com/games/wingspan/)
that follows [Gymnasium](https://gymnasium.farama.org/) spec.

Purpose of this repo is entirely for educational purposes,
and it should not be used instead of playing actual wingspan.
If that's what you are planning to do, see [Stonemaier's page](https://stonemaiergames.com/games/wingspan/digital-versions/) to see possibilities.

The main reason why this was made, is because I am interested if there is a way to provide a statistical analysis backing strategies listed in [Wingsplain](https://wingsplain.com/).
It's also a cool project, and my work essentially forbids me from working on anything robotics related :man_shrugging:

!!! warning

    Package is still not stable, and functionality is not compatible with the spec of Gymnasium.
    It also has missing features that do not match fully to wingspan game yet.
    See roadmap in a section below.

## Installation
You can `pip install wingspan-gym`.

!!! note

    If you are installing directly from python sdist (i.e. you are not on OS/arch I've pre-built the package for). You might need to [install rustup](https://rustup.rs/) first.


## Roadmap
As mentioned above the repo is still very much in a development state.
Following roadmap lists all of the features, roughly in order of priority (though things might change):

1.  Create a better test suite for verifying code correctness (currently done mostly by hand).
2.  Implement all of the missing core bird actions.
3.  Implement all of the end of round goals.
4.  Somewhere here I want to implement a very basic, ncurses based terminal UI to play the game for easier testing, and debugging edge-cases.
5.  Define state, and expose it to python.
6.  Have spec match one of the gymnasium environment
7.  *This is probably where v1 is cut*
8.  Implement expansions in order of release. Each will probably result in major version bumps, due to breaking changes.

Anything beyond this is out of scope for the project. I want to provide some examples of running the code with a RL agent, but haven't decided on details yet.
