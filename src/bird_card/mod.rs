
pub mod bird_card_constants;
pub mod bird_card_impl;

pub use bird_card_impl::*;
pub use bird_card_constants::*;
use strum::IntoEnumIterator;

use crate::expansion::Expansion;


pub(crate) fn get_deck(expansions: &[Expansion]) -> Vec<BirdCard> {
    if expansions.len() != 0 {
        todo!("Only core is supported so far. Expansions add new logic which we have not implemented yet.")
    }

    Vec::from_iter(BirdCard::iter())
}
