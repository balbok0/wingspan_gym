
pub mod bird_card_constants;
pub mod bird_card_impl;

pub use bird_card_impl::*;
pub use bird_card_constants::*;
use strum::IntoEnumIterator;

use crate::{expansion::Expansion, food::Foods};


pub(crate) fn get_deck(expansions: &[Expansion]) -> Vec<BirdCard> {
    if expansions.len() != 0 {
        todo!("Only core is supported so far. Expansions add new logic which we have not implemented yet.")
    }

    Vec::from_iter(BirdCard::iter())
}

pub(crate) fn is_enough_food_to_play_a_card(card: &BirdCard, player_food: &Foods) -> bool {
    let (food_req, total_food_needed, is_cost_alt) = card.cost();

    let total_food: u8 = player_food.iter().sum();
    if total_food < *total_food_needed {
        return false;
    }

    match is_cost_alt {
        crate::food::CostAlternative::Yes => {
            food_req.iter().zip(player_food).map(
                |(req, res)| {
                    req.map_or(false, |req| req <= *res)
                }
            ).fold(false, |a, b| a || b)
        },
        crate::food::CostAlternative::No => {
            food_req.iter().zip(player_food).map(
                |(req, res)| {
                    req.map_or(true, |req| req <= *res)
                }
            ).fold(true, |a, b| a && b)
        }
    }
}