use pyo3::prelude::*;

use crate::{
    bonus_card::BonusCard, expansion::Expansion, food::BirdCardCost, habitat::Habitat,
    nest::NestType,
};

use super::{BirdCard, BirdCardColor};

#[pymethods]
impl BirdCard {
    #[getter(index)]
    pub fn get_index(&self) -> u16 {
        self.index()
    }

    #[getter(name)]
    pub fn get_name(&self) -> &'static str {
        self.name()
    }

    #[getter(cost)]
    pub fn get_cost(&self) -> BirdCardCost {
        *self.cost()
    }

    #[getter(color)]
    pub fn get_color(&self) -> BirdCardColor {
        *self.color()
    }

    #[getter(points)]
    pub fn get_points(&self) -> u8 {
        self.points()
    }

    #[getter(habitats)]
    pub fn get_habitats(&self) -> Vec<Habitat> {
        self.habitats().to_vec()
    }

    #[getter(wingspan)]
    pub fn get_wingspan(&self) -> Option<u16> {
        self.wingspan()
    }

    #[getter(egg_capacity)]
    pub fn get_egg_capacity(&self) -> u8 {
        self.egg_capacity()
    }

    #[getter(nest_type)]
    pub fn get_nest_type(&self) -> NestType {
        *self.nest_type()
    }

    #[getter(is_predator)]
    pub fn get_is_predator(&self) -> bool {
        self.is_predator()
    }

    #[getter(expansion)]
    pub fn get_expansion(&self) -> Expansion {
        self.expansion()
    }

    #[getter(bonus_card_membership)]
    pub fn get_bonus_card_membership(&self) -> Vec<BonusCard> {
        self.bonus_card_membership()
    }
}
