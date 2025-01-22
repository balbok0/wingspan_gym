use crate::action::Action;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Habitat {
    Forest,
    Grassland,
    Wetland,
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
            Habitat::Forest => Action::DoThen(Box::new(Action::DiscardBirdCard), Box::new(Action::GetFood)),
            Habitat::Grassland => Action::DoThen(Box::new(Action::DiscardFood), Box::new(Action::GetEgg)),
            Habitat::Wetland => Action::DoThen(Box::new(Action::DiscardEgg), Box::new(Action::GetBirdCard)),
        }
    }
}