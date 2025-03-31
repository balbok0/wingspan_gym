use crate::action::Action;
use pyo3::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(eq, eq_int)]
pub enum Habitat {
    Forest,
    Grassland,
    Wetland,
}

pub const HABITATS: [Habitat; 3] = [Habitat::Forest, Habitat::Grassland, Habitat::Wetland];

impl From<usize> for Habitat {
    fn from(value: usize) -> Self {
        match value {
            0 => Habitat::Forest,
            1 => Habitat::Grassland,
            2 => Habitat::Wetland,
            x => panic!("Got habitat idx {x}, but only 0, 1, 2 are allowed."),
        }
    }
}

impl From<u8> for Habitat {
    fn from(value: u8) -> Self {
        Habitat::from(value as usize)
    }
}

impl Habitat {
    pub fn action(&self) -> Action {
        match self {
            Habitat::Forest => Action::GetFood,
            Habitat::Grassland => Action::GetEgg,
            Habitat::Wetland => Action::GetBirdCard,
        }
    }

    pub fn optional_action(&self) -> Action {
        match self {
            Habitat::Forest => {
                Action::DoThen(Box::new(Action::DiscardBirdCard), Box::new(Action::GetFood))
            }
            Habitat::Grassland => {
                Action::DoThen(Box::new(Action::DiscardFood), Box::new(Action::GetEgg))
            }
            Habitat::Wetland => {
                Action::DoThen(Box::new(Action::DiscardEgg), Box::new(Action::GetBirdCard))
            }
        }
    }
}
