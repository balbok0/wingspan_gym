use rand::{rngs::StdRng, Rng};
use pyo3::prelude::*;

use crate::{
    bird_card::{BeakDirection, BirdCardColor},
    expansion::Expansion,
    food::CostAlternative,
    habitat::Habitat,
    nest::NestType,
    wingspan_env::WingspanEnv,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[pyclass(eq, eq_int)]
pub enum EndOfRoundScoring {
    Competitive,
    Friendly,
}

#[derive(Debug, Clone, Copy)]
pub enum EndOfRoundGoal {
    // Core
    BirdsInHabitat(Habitat),
    BirdsWithEggWithNestType(NestType),
    EggsInNestType(NestType),
    EggsInHabitat(Habitat),
    BirdsPlayedTotal, // Total on board
    SetsOfEggs,       // 1 = 1 egg in each habitat

    // European
    BirdsWithTuckedCards,
    BirdsInOneRow,
    BirdsWithNoEggs,
    BirdsInHand,
    BirdsGEPoints(u8),
    BirdsWithColor(BirdCardColor),
    FoodInPlayerSupply,
    FilledColumns,
    FoodCostPlayed,

    // Oceania
    CubesOnPlayABird, // Birds played this round
    BeakPointingLeft,
    BeakPointingRight,
    BirdsLEPoints(u8),
    FoodsInBirdCosts([bool; 5]),
}

impl EndOfRoundGoal {
    pub fn get_num_matching(&self, env: &WingspanEnv, player_idx: usize) -> usize {
        let player = env.get_player(player_idx);

        match self {
            EndOfRoundGoal::BirdsInHabitat(habitat) => {
                player.get_mat().get_row(habitat).get_birds().len()
            }
            EndOfRoundGoal::BirdsWithEggWithNestType(nest_type) => {
                player.get_mat().get_birds_with_nest_type(nest_type).len()
            }
            EndOfRoundGoal::EggsInNestType(nest_type) => {
                let mat = player.get_mat();
                mat.get_birds_with_nest_type(nest_type)
                    .iter()
                    .map(|(habitat, bird_idx)| mat.get_row(habitat).get_eggs()[*bird_idx])
                    .sum::<u8>() as usize
            }
            EndOfRoundGoal::EggsInHabitat(habitat) => player
                .get_mat()
                .get_row(habitat)
                .get_eggs()
                .iter()
                .sum::<u8>() as usize,
            EndOfRoundGoal::BirdsPlayedTotal => player
                .get_mat()
                .rows()
                .map(|row| row.get_birds().len())
                .iter()
                .sum(),
            EndOfRoundGoal::SetsOfEggs => player
                .get_mat()
                .rows()
                .map(|row| row.get_eggs().iter().sum::<u8>() as usize)
                .into_iter()
                .min()
                .unwrap(),
            EndOfRoundGoal::BirdsWithTuckedCards => player
                .get_mat()
                .rows()
                .map(|row| row.get_tucked_cards().iter().filter(|tc| **tc > 0).count())
                .iter()
                .sum(),
            EndOfRoundGoal::BirdsInOneRow => player
                .get_mat()
                .rows()
                .map(|row| row.get_birds().len())
                .into_iter()
                .max()
                .unwrap(),
            EndOfRoundGoal::BirdsWithNoEggs => player
                .get_mat()
                .rows()
                .map(|row| row.get_eggs().iter().filter(|ec| **ec == 0).count())
                .into_iter()
                .sum(),
            EndOfRoundGoal::BirdsInHand => player.get_bird_cards().len(),
            EndOfRoundGoal::BirdsGEPoints(pts_thres) => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .filter(|bc| bc.points() >= *pts_thres)
                        .count()
                })
                .into_iter()
                .sum(),
            EndOfRoundGoal::BirdsWithColor(BirdCardColor::White) => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .filter(|bc| {
                            matches!(bc.color(), BirdCardColor::White | BirdCardColor::None)
                        })
                        .count()
                })
                .into_iter()
                .sum(),
            EndOfRoundGoal::BirdsWithColor(bird_card_color) => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .filter(|bc| bc.color() == bird_card_color)
                        .count()
                })
                .into_iter()
                .sum(),
            EndOfRoundGoal::FoodInPlayerSupply => player.get_foods().iter().sum::<u8>() as usize,
            EndOfRoundGoal::FilledColumns => player.get_mat().get_columns().len(),
            EndOfRoundGoal::FoodCostPlayed => player
                .get_mat()
                .rows()
                .map(|row| row.get_birds().iter().map(|bc| bc.cost().1).sum::<u8>() as usize)
                .into_iter()
                .sum(),
            EndOfRoundGoal::CubesOnPlayABird => todo!(),
            EndOfRoundGoal::BeakPointingLeft => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .filter(|bc| bc.beak_direction() == BeakDirection::Left)
                        .count()
                })
                .into_iter()
                .sum(),
            EndOfRoundGoal::BeakPointingRight => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .filter(|bc| bc.beak_direction() == BeakDirection::Right)
                        .count()
                })
                .into_iter()
                .sum(),
            EndOfRoundGoal::BirdsLEPoints(pts_thres) => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .filter(|bc| bc.points() <= *pts_thres)
                        .count()
                })
                .into_iter()
                .sum(),
            EndOfRoundGoal::FoodsInBirdCosts(foods_to_include) => player
                .get_mat()
                .rows()
                .map(|row| {
                    row.get_birds()
                        .iter()
                        .map(|bc| {
                            let bc_cost = bc.cost();
                            let food_cost_iter = bc_cost.0.iter().zip(foods_to_include).filter_map(
                                |(food_cost, include_food)| {
                                    if *include_food {
                                        *food_cost
                                    } else {
                                        None
                                    }
                                },
                            );

                            if bc_cost.2 == CostAlternative::No {
                                food_cost_iter.sum()
                            } else {
                                food_cost_iter.max().unwrap_or_default()
                            }
                        })
                        .sum::<u8>() as usize
                })
                .into_iter()
                .sum(),
        }
    }
}

type Tile = [EndOfRoundGoal; 2];

const CORE_CARDS: [Tile; 8] = [
    [
        EndOfRoundGoal::BirdsInHabitat(Habitat::Forest),
        EndOfRoundGoal::EggsInHabitat(Habitat::Forest),
    ],
    [
        EndOfRoundGoal::BirdsInHabitat(Habitat::Grassland),
        EndOfRoundGoal::EggsInHabitat(Habitat::Grassland),
    ],
    [
        EndOfRoundGoal::BirdsInHabitat(Habitat::Wetland),
        EndOfRoundGoal::EggsInHabitat(Habitat::Wetland),
    ],
    [
        EndOfRoundGoal::BirdsWithEggWithNestType(NestType::Bowl),
        EndOfRoundGoal::EggsInNestType(NestType::Bowl),
    ],
    [
        EndOfRoundGoal::BirdsWithEggWithNestType(NestType::Cavity),
        EndOfRoundGoal::EggsInNestType(NestType::Cavity),
    ],
    [
        EndOfRoundGoal::BirdsWithEggWithNestType(NestType::Platform),
        EndOfRoundGoal::EggsInNestType(NestType::Platform),
    ],
    [
        EndOfRoundGoal::BirdsWithEggWithNestType(NestType::Ground),
        EndOfRoundGoal::EggsInNestType(NestType::Ground),
    ],
    [EndOfRoundGoal::SetsOfEggs, EndOfRoundGoal::BirdsPlayedTotal],
];

const EUROPEAN_CARDS: [Tile; 5] = [
    [
        EndOfRoundGoal::BirdsWithTuckedCards,
        EndOfRoundGoal::FoodCostPlayed,
    ],
    [EndOfRoundGoal::BirdsInOneRow, EndOfRoundGoal::FilledColumns],
    [
        EndOfRoundGoal::BirdsWithNoEggs,
        EndOfRoundGoal::BirdsGEPoints(5),
    ],
    [
        EndOfRoundGoal::BirdsInHand,
        EndOfRoundGoal::FoodInPlayerSupply,
    ],
    [
        EndOfRoundGoal::BirdsWithColor(BirdCardColor::Brown),
        EndOfRoundGoal::BirdsWithColor(BirdCardColor::White),
    ],
];

const OCEANIA_CARDS: [Tile; 4] = [
    [
        EndOfRoundGoal::CubesOnPlayABird,
        EndOfRoundGoal::BirdsLEPoints(3),
    ],
    [
        EndOfRoundGoal::BeakPointingLeft,
        EndOfRoundGoal::BeakPointingRight,
    ],
    [
        // Seed + Fruit
        EndOfRoundGoal::FoodsInBirdCosts([false, true, false, true, false]),
        // Invertebrate
        EndOfRoundGoal::FoodsInBirdCosts([true, false, false, false, false]),
    ],
    [
        // Rodent + Fish
        EndOfRoundGoal::FoodsInBirdCosts([false, false, true, false, true]),
        // (Empty side, so repeat of side above) Rodent + Fish
        EndOfRoundGoal::FoodsInBirdCosts([false, false, true, false, true]),
    ],
];

fn get_end_of_round_deck(expansions: &[Expansion]) -> Vec<Tile> {
    let mut result = vec![];

    let expansion_decks: [(Expansion, &[[EndOfRoundGoal; 2]]); 3] = [
        (Expansion::Core, &CORE_CARDS),
        (Expansion::European, &EUROPEAN_CARDS),
        (Expansion::Oceania, &OCEANIA_CARDS),
    ];

    for (expansion, deck) in expansion_decks.iter() {
        if expansions.contains(expansion) {
            result.extend_from_slice(deck);
        }
    }

    result
}

pub(crate) fn sample_end_of_round_goals(
    expansions: &[Expansion],
    num_rounds: usize,
    rng: &mut StdRng,
) -> Vec<EndOfRoundGoal> {
    if expansions.len() != 1 && expansions.first().unwrap() != &Expansion::Core {
        todo!("Only core is supported so far. Expansions add new logic which we have not implemented yet.")
    }

    let mut tiles = get_end_of_round_deck(expansions);
    let mut result = vec![];

    for _ in 0..num_rounds {
        let idx = rng.gen_range(0..tiles.len());
        let tile = tiles.remove(idx);
        let side_idx = rng.gen_range(0..2usize);
        result.push(tile[side_idx]);
    }

    result
}
