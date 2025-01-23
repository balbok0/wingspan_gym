use crate::{action::Action, bird_card::BirdCard, error::{WingError, WingResult}, habitat::Habitat};

type EggsRow = [u8; 5];


#[derive(Debug, Clone)]
pub struct MatRow {
    pub birds: Vec<BirdCard>,
    eggs: EggsRow,
    eggs_cap: EggsRow,
}

impl Default for MatRow {
    fn default() -> Self {
        Self {
            birds: Vec::with_capacity(5),
            eggs: [0, 0, 0, 0, 0],
            eggs_cap: [0, 0, 0, 0, 0],
        }
    }
}

impl MatRow {
    pub fn row_to_play(&self) -> Option<u8> {
        if self.birds.len() == 5 {
            return None;
        } else {
            return Some(self.birds.len() as u8)
        }
    }

    pub fn get_bird_actions(&self) -> Vec<Action> {
        // TODO: Implement bird actions and then populate this stuff
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct PlayerMat {
    forest: MatRow,
    grassland: MatRow,
    wetland: MatRow,
    num_eggs: u8,
    eggs_cap: u8,
}

impl Default for PlayerMat {
    fn default() -> Self {
        Self {
            forest: Default::default(),
            grassland: Default::default(),
            wetland: Default::default(),
            num_eggs: 0,
            eggs_cap: 0,
        }
    }
}

impl PlayerMat {
    fn get_row(&self, habitat: &Habitat) -> &MatRow {
        match habitat {
            Habitat::Forest => {
                &self.forest
            },
            Habitat::Grassland => {
                &self.grassland
            },
            Habitat::Wetland => {
                &self.wetland
            },
        }
    }

    fn get_row_mut(&mut self, habitat: &Habitat) -> &mut MatRow {
        match habitat {
            Habitat::Forest => {
                &mut self.forest
            },
            Habitat::Grassland => {
                &mut self.grassland
            },
            Habitat::Wetland => {
                &mut self.wetland
            },
        }
    }

    pub fn playable_habitats(&self, card: &BirdCard) -> Vec<Habitat> {
        card.habitats().iter().filter(|habitat| {
            let hab_row = self.get_row(habitat);

            if let Some(row) = hab_row.row_to_play() {
                // There is a place in habitat.
                // Check if we have enough eggs
                let egg_req = (row + 1) / 2;

                if egg_req <= self.num_eggs {
                    // Not enough eggs
                    return false;
                }

                // Eggs are satisfied and there is a place
                return true;
            } else {
                // No place in a habitat
                return false
            }
        }).cloned().collect()
    }

    pub fn get_actions(&self, habitat: &Habitat) -> Vec<Action> {
        let hab_action = habitat.action();
        let hab_row = self.get_row(&habitat);

        let mut result = hab_row.get_bird_actions();

        let num_actions = if habitat == &Habitat::Grassland  {
            2
        } else {
            1
        } + hab_row.birds.len() / 2;

        result.extend((0..num_actions).map(|_| hab_action.clone()));

        if hab_row.birds.len() % 2 == 1 {
            result.push(habitat.optional_action())
        }

        result
    }

    pub fn can_place_egg(&self) -> bool {
        self.num_eggs < self.eggs_cap
    }

    pub fn can_discard_egg(&self) -> bool {
        self.num_eggs > 0
    }

    pub fn put_bird_card(&mut self, bird_card: BirdCard, habitat: &Habitat) -> WingResult<()> {
        let row = self.get_row_mut(habitat);
        if row.birds.len() >= 5 {
            Err(WingError::InvalidAction)
        } else {
            row.birds.push(bird_card);
            Ok(())
        }
    }
}
