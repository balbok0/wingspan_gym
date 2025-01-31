use itertools::Itertools as _;

use crate::{habitat::Habitat, nest::NestType, player::Player};

use super::BonusCard;

enum ChainState {
    Unknown,
    Increasing,
    Decreasing,
}

impl BonusCard {
    pub fn get_count_of_matching(&self, player: &Player) -> usize {
        match self {
            BonusCard::BreedingManager => {
                // Birds that have at least 4 eggs laid on them
                player.get_mat().rows().iter()
                    .flat_map(|mat_row| {
                        mat_row.get_eggs()
                            .iter()
                            .filter(|eggs| **eggs >= 4)
                    })
                    .count()
            },
            BonusCard::Ecologist => {
                // Birds in your habitat with the fewest birds.
                player.get_mat().rows().iter()
                    .map(|mat_row| mat_row.get_birds().len())
                    .min()
                    .unwrap()
            },
            BonusCard::Oologist => {
                // Birds that have at least 1 egg laid on them
                player.get_mat().rows().iter()
                    .flat_map(|mat_row| {
                        mat_row.get_eggs()
                            .iter()
                            .filter(|eggs| **eggs >= 1)
                    })
                    .count()
            },
            BonusCard::VisionaryLeader => {
                // Bird cards in hand at end of game
                player.get_bird_cards().len()
            },
            BonusCard::Behaviorist => {
                // For each column that contains birds with 3 different power colors:

                // None (no-color) counts as White
                player
                    .get_mat()
                    .get_columns()
                    .iter()
                    .filter(
                        |col|
                        // Get number of unique colors per-column, make sure it is 3
                        col
                            .iter()
                            .filter_map(|b| b.map(|b| b.color()))
                            .unique_by(|col| col.unique_id())
                            .count() == 3
                    )
                    .count()
            },
            BonusCard::CitizenScientist => {
                // Birds with tucked cards
                player.get_mat().rows().iter()
                    .flat_map(|mat_row| {
                        mat_row.get_tucked_cards()
                            .iter()
                            .filter(|tc| **tc >= 1)
                    })
                    .count()
            },
            BonusCard::Ethologist => {
                // In any one habitat: 2pts per Power Color
                player.get_mat().rows().iter()
                    .flat_map(|mat_row| {
                        mat_row.get_birds()
                            .iter()
                            .map(|b| b.color())
                            .unique_by(|b| b.unique_id())
                    })
                    .count()
            },
            BonusCard::ForestDataAnalyst | BonusCard::GrasslandDataAnalyst | BonusCard::WetlandDataAnalyst => {
                // Consecutive birds in [habitat] with ascending or descending wingspans
                let habitiat = match self {
                    BonusCard::ForestDataAnalyst => Habitat::Forest,
                    BonusCard::GrasslandDataAnalyst => Habitat::Grassland,
                    BonusCard::WetlandDataAnalyst => Habitat::Wetland,
                    _ => panic!("Wut")
                };
                let birds = player.get_mat().get_row(&habitiat).get_birds();

                // 2 or less birds is pre-defined result
                if birds.len() <= 2 {
                    return birds.len();
                }

                // TODO: Test this with wildcards!
                let mut longest_chain_count = 0;

                // Find longest chain
                for chain_starting_idx in 0..birds.len() {
                    let mut cur_chain_count = 0;
                    let mut chain_dir = ChainState::Unknown;
                    let mut prev_wingspan = None;

                    // It's so short birds wise left that 2 birds is the answer
                    if birds.len() - chain_starting_idx < 3 {
                        longest_chain_count = longest_chain_count.max(2);
                        break;
                    }

                    for bird in birds[chain_starting_idx..].iter() {
                        let cur_wingspan = bird.wingspan();

                        // Previous ref point (thus direction) is not yet known
                        if prev_wingspan == None {
                            cur_chain_count += 1;
                            prev_wingspan = cur_wingspan;
                            continue;
                        }

                        // Wildcard wingspan. Always works
                        if cur_wingspan == None {
                            cur_chain_count += 1;
                            continue;
                        }

                        // We know that both of these are set.
                        let prev_known_wingspan = prev_wingspan.unwrap();
                        let cur_known_wingspan = cur_wingspan.unwrap();

                        match chain_dir {
                            ChainState::Unknown => {
                                cur_chain_count += 1;
                                prev_wingspan = cur_wingspan;
                                if prev_known_wingspan > cur_known_wingspan {
                                    chain_dir = ChainState::Decreasing;
                                } else if prev_known_wingspan < cur_known_wingspan {
                                    chain_dir = ChainState::Increasing;
                                } else {
                                    // They got the same wingspan
                                    chain_dir = ChainState::Unknown;
                                }
                            },
                            ChainState::Decreasing => {
                                if prev_known_wingspan >= cur_known_wingspan {
                                    cur_chain_count += 1;
                                    prev_wingspan = cur_wingspan;
                                } else {
                                    break;
                                }
                            },
                            ChainState::Increasing => {
                                if prev_known_wingspan <= cur_known_wingspan {
                                    cur_chain_count += 1;
                                    prev_wingspan = cur_wingspan;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    longest_chain_count = longest_chain_count.max(cur_chain_count)
                }

                longest_chain_count
            },
            BonusCard::MechanicalEngineer => {
                // Sets of the 4 nest types / 1 set = [bowl] [cavity] [ground] [platform]
                // Each star nest can be treated as any 1 nest type. No card can be part of more than 1 set.
                player.get_mat().rows().iter()
                    .filter(|mat_row| {
                        // Unique standard nest types
                        let unique_types = mat_row.get_birds()
                            .iter()
                            .map(|b| b.nest_type())
                            .filter_map(|nt| {
                                if nt == &NestType::Wild || nt == &NestType::Wild {
                                    None
                                } else {
                                    Some(nt)
                                }
                            })
                            .unique()
                            .count();
                        // star nest types
                        let star_count = mat_row.get_birds()
                            .iter()
                            .filter(|b| b.nest_type() == &NestType::Wild)
                            .count();

                        unique_types + star_count >= 4
                    })
                    .count()
            },
            BonusCard::SiteSelectionExpert => {
                // Columns with a matching pair or trio of nests
                todo!("This bonus card is not supported yet. This is because there are two conditions, with 2 different counts on it.")
                // player
                //     .get_mat()
                //     .get_columns()
                //     .iter()
                //     .map(
                //         |bc_col| bc_col
                //     )
            },
            BonusCard::AvianTheriogenologist => {
                // Birds with completely full nests
                player.get_mat().rows().iter()
                    .map(
                        |row|
                        row.get_eggs().iter().zip(row.get_eggs_cap()).filter(|(eggs, eggs_cap)| {
                            eggs == eggs_cap && **eggs_cap > 0
                        }).count()
                    )
                    .sum()
            },
            BonusCard::ForestPopulationMonitor | BonusCard::GrasslandPopulationMonitor | BonusCard::WetlandPopulationMonitor => {
                // Different nest types in [habitat]
                // You may count each [star] nest as any other type or as a fifth type.
                let habitat = match self {
                    BonusCard::ForestPopulationMonitor => Habitat::Forest,
                    BonusCard::GrasslandPopulationMonitor => Habitat::Grassland,
                    BonusCard::WetlandPopulationMonitor => Habitat::Wetland,
                    _ => panic!("wut"),
                };
                let mat_row = player.get_mat().get_row(&&habitat);

                // Unique standard nest types
                let unique_types = mat_row.get_birds()
                    .iter()
                    .map(|b| b.nest_type())
                    .filter_map(|nt| {
                        if nt == &NestType::Wild || nt == &NestType::Wild {
                            None
                        } else {
                            Some(nt)
                        }
                    })
                    .unique()
                    .count();

                // star nest types
                let star_count = mat_row.get_birds()
                    .iter()
                    .filter(|b| b.nest_type() == &NestType::Wild)
                    .count();

                unique_types + star_count
            },
            BonusCard::ForestRanger | BonusCard::GrasslandRanger | BonusCard::WetlandRanger => {
                // Consecutive birds in [habitat] with ascending or descending scores
                let habitat = match self {
                    BonusCard::ForestPopulationMonitor => Habitat::Forest,
                    BonusCard::GrasslandPopulationMonitor => Habitat::Grassland,
                    BonusCard::WetlandPopulationMonitor => Habitat::Wetland,
                    _ => panic!("wut"),
                };
                let birds = player.get_mat().get_row(&&habitat).get_birds();

                let mut longest_chain_count = 0;

                // Find longest chain
                for chain_starting_idx in 0..birds.len() {
                    let mut cur_chain_count = 0;
                    let mut chain_dir = ChainState::Unknown;
                    let mut prev_score = None;

                    // It's so short birds wise left that 2 birds is the answer
                    if birds.len() - chain_starting_idx < 3 {
                        longest_chain_count = longest_chain_count.max(2);
                        break;
                    }

                    for bird in birds[chain_starting_idx..].iter() {
                        let cur_score = bird.points();

                        // Previous ref point (thus direction) is not yet known
                        if prev_score == None {
                            cur_chain_count += 1;
                            prev_score = Some(cur_score);
                            continue;
                        }
                        let prev_known_score = prev_score.unwrap();

                        match chain_dir {
                            ChainState::Unknown => {
                                cur_chain_count += 1;

                                if prev_known_score > cur_score {
                                    chain_dir = ChainState::Decreasing;
                                } else if prev_known_score < cur_score {
                                    chain_dir = ChainState::Increasing;
                                } else {
                                    // They got the same points score
                                    // This is a breaking condition for rangers
                                    break;
                                }
                            },
                            ChainState::Decreasing => {
                                if prev_known_score > cur_score {
                                    cur_chain_count += 1;
                                    prev_score = Some(cur_score);
                                } else {
                                    break;
                                }
                            },
                            ChainState::Increasing => {
                                if prev_known_score < cur_score {
                                    cur_chain_count += 1;
                                    prev_score = Some(cur_score);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    longest_chain_count = longest_chain_count.max(cur_chain_count)
                }

                longest_chain_count
            },
            BonusCard::PelletDissector => {
                // [fish] and [rodent] tokens cached on your birds
                todo!()
            },
            BonusCard::WinterFeeder => {
                // Food remaining in your supply at end of game
                player.get_foods().iter().cloned().sum::<u8>() as usize
            },
            _ => {
                // All of the bonus cards that have a column in birds sheet
                player
                    .get_mat()
                    .rows()
                    .iter()
                    .map(|mr| mr.get_birds())
                    .flatten()
                    .filter(|bc| bc.bonus_card_membership().contains(&self))
                    .count()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{bird_card::BirdCard, player_mat::{MatRow, PlayerMat}};

    use super::*;

    fn make_player_from_cards_on_table(forest_cards: Vec<BirdCard>, grassland_cards: Vec<BirdCard>, wetland_cards: Vec<BirdCard>) -> Player {
        let [forest, grassland, wetland] = [forest_cards, grassland_cards, wetland_cards].map(|cards| {
            MatRow::new_test(
                vec![],
                0,
                cards,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        });
        let mat = PlayerMat::new_test(forest, grassland, wetland, 0, 0);

        let player = Player::new_test(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            mat,
            Default::default(),
            Default::default(),
        );

        player
    }


    macro_rules! regular_cards_tests {
        ($(($name:ident: $bonus:expr, $forest:expr, $grassland:expr, $wetland:expr, $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let player = make_player_from_cards_on_table($forest, $grassland, $wetland);

                let actual = $bonus.get_count_of_matching(&player);
                assert_eq!($expected, actual);
            }
        )*
        }
    }

    regular_cards_tests!(
        (empty: BonusCard::Anatomist, vec![], vec![], vec![], 0),
        (anatomist: BonusCard::Anatomist, vec![BirdCard::AshThroatedFlycatcher], vec![BirdCard::BarrowsGoldeneye], vec![], 2),
        (
            historian: BonusCard::Historian,
            vec![BirdCard::AshThroatedFlycatcher, BirdCard::AnnasHummingbird, BirdCard::BairdsSparrow],
            vec![],
            vec![BirdCard::BarrowsGoldeneye],
            3
        ),

    );
    // #[test]
    // fn test_get_count_of_matching_reg_bonus_card() {
    //     make_player_from_cards_on_table(forest_cards, grassland_cards, wetland_cards)

    //     BonusCard::Anatomist.get_count_of_matching(player)
    // }
}