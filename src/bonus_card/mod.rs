mod bonus_card_impl;
mod bonus_card_manual;
mod bonus_scoring;
mod bonus_card_python;

use std::collections::HashSet;

pub use bonus_card_impl::BonusCard;
pub use bonus_scoring::*;
use strum::IntoEnumIterator as _;

use crate::expansion::Expansion;

pub(crate) fn get_deck(expansions: &[Expansion]) -> Vec<BonusCard> {
    if expansions.len() != 1 && expansions.first().unwrap() != &Expansion::Core {
        todo!("Only core is supported so far. Expansions add new logic which we have not implemented yet.")
    }

    let expansions = HashSet::<Expansion>::from_iter(expansions.iter().cloned());

    BonusCard::iter()
        .filter(|bc| expansions.contains(&bc.expansion()))
        .collect()
}
