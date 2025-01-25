use crate::{action::Action, bird_card::BirdCard, error::{WingError, WingResult}, habitat::Habitat};

type BirdResourceRow = [u8; 5];


#[derive(Debug, Clone)]
pub struct MatRow {
    pub birds: Vec<BirdCard>,
    pub tucked_cards: BirdResourceRow,
    pub cached_food: BirdResourceRow,
    pub eggs: BirdResourceRow,
    pub eggs_cap: BirdResourceRow,
}

impl Default for MatRow {
    fn default() -> Self {
        Self {
            birds: Vec::with_capacity(5),
            tucked_cards: [0, 0, 0, 0, 0],
            cached_food: [0, 0, 0, 0, 0],
            eggs: [0, 0, 0, 0, 0],
            eggs_cap: [0, 0, 0, 0, 0],
        }
    }
}

impl MatRow {
    pub fn col_to_play(&self) -> Option<u8> {
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

    pub fn num_spots_to_place_eggs(&self) -> usize {
        self.eggs.iter()
            .zip(self.eggs_cap)
            .filter(|(eggs, cap)| *eggs < cap)
            .count()
    }

    pub fn num_spots_to_discard_eggs(&self) -> usize {
        self.eggs.iter()
            .filter(|eggs| **eggs > 0)
            .count()
    }

    pub fn place_egg(&mut self, idx: usize) -> Result<(), usize> {
        let mut count = 0;

        for (col_idx, (egg, cap)) in self.eggs.iter().zip(self.eggs_cap).enumerate() {
            let egg = *egg;
            if egg < cap {
                // Valid spot to put egg in
                if count == idx {
                    // This is the requested spot
                    self.eggs[col_idx] += 1;
                    return Ok(())
                } else {
                    // Not yet the requested spot
                    count += 1;
                }
            }
        }

        // Requested spot not found, so return number of valid spots in this row
        Err(count)
    }

    pub fn discard_egg(&mut self, idx: usize) -> Result<(), usize> {
        let mut count = 0;

        for (col_idx, egg) in self.eggs.iter().enumerate() {
            let egg = *egg;
            if egg > 0 {
                // Valid spot to discard egg from
                if count == idx {
                    // This is the requested spot
                    self.eggs[col_idx] -= 1;
                    return Ok(())
                } else {
                    // Not yet the requested spot
                    count += 1;
                }
            }
        }

        // Requested spot not found, so return number of valid spots in this row
        Err(count)
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
    pub fn get_row(&self, habitat: &Habitat) -> &MatRow {
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

            if let Some(col) = hab_row.col_to_play() {
                // There is a place in habitat.
                // Check if we have enough eggs
                let egg_req = (col + 1) / 2;

                if egg_req > self.num_eggs {
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

    pub fn get_actions_from_habitat_action(&self, habitat: &Habitat) -> Vec<Action> {
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

    pub fn num_spots_to_place_eggs(&self) -> usize {
        self.rows().map(|a| MatRow::num_spots_to_place_eggs(a)).iter().sum()
    }

    pub fn num_spots_to_discard_eggs(&self) -> usize {
        self.rows().map(|a| MatRow::num_spots_to_discard_eggs(a)).iter().sum()
    }

    pub fn place_egg(&mut self, idx: u8) -> WingResult<()> {
        let idx = idx as usize;
        let mut cur_action_count = 0;
        for hab_row in [&mut self.forest, &mut self.grassland, &mut self.wetland] {
            match hab_row.place_egg(idx - cur_action_count) {
                Ok(()) => {
                    self.num_eggs += 1;
                    return Ok(());
                }
                Err(num_actions_in_row) => {
                    cur_action_count += num_actions_in_row;
                }
            }
        }

        // No places found to place eggs, so this was an invalid action
        Err(WingError::InvalidAction)
    }

    pub fn discard_egg(&mut self, idx: u8) -> WingResult<()> {
        let idx = idx as usize;
        let mut cur_action_count = 0;
        for hab_row in [&mut self.forest, &mut self.grassland, &mut self.wetland] {
            match hab_row.discard_egg(idx - cur_action_count) {
                Ok(()) => {
                    self.num_eggs -= 1;
                    return Ok(());
                }
                Err(num_actions_in_row) => {
                    cur_action_count += num_actions_in_row;
                }
            }
        }

        // No places found to place eggs, so this was an invalid action
        Err(WingError::InvalidAction)
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
            let egg_cap = bird_card.egg_capacity();
            row.eggs_cap[row.birds.len()] += egg_cap;
            row.birds.push(bird_card);
            self.eggs_cap += egg_cap;
            Ok(())
        }
    }

    pub fn rows(&self) -> [&MatRow; 3] {
        [&self.forest, &self.grassland, &self.wetland]
    }

    pub fn egg_count(&self) -> u8 {
        self.num_eggs
    }
}
