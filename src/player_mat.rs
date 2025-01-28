use crate::{action::Action, bird_card::BirdCard, error::{WingError, WingResult}, habitat::Habitat};

type BirdResourceRow = [u8; 5];


#[derive(Debug, Clone)]
pub struct MatRow {
    // Mapping from column idx -> index in birds. This is because some birds can cover multiple places
    bird_col_idxs: Vec<usize>,
    next_col_to_play: usize,
    birds: Vec<BirdCard>,
    tucked_cards: BirdResourceRow,
    cached_food: Vec<BirdResourceRow>,
    eggs: BirdResourceRow,
    eggs_cap: BirdResourceRow,
}

impl Default for MatRow {
    fn default() -> Self {
        Self {
            birds: Vec::with_capacity(5),
            bird_col_idxs: Vec::with_capacity(5),
            next_col_to_play: 0,
            tucked_cards: [0, 0, 0, 0, 0],
            cached_food: Vec::with_capacity(5),
            eggs: [0, 0, 0, 0, 0],
            eggs_cap: [0, 0, 0, 0, 0],
        }
    }
}

impl MatRow {
    pub fn col_to_play(&self) -> Option<u8> {
        if self.next_col_to_play >= 5 {
            return None;
        } else {
            return Some(self.next_col_to_play as u8)
        }
    }

    pub fn bird_at_idx(&self, idx: usize) -> Option<BirdCard> {
        Some(*self.birds.get(*self.bird_col_idxs.get(idx)?)?)
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

    pub fn get_birds(&self) -> &Vec<BirdCard> {
        &self.birds
    }

    pub fn get_eggs(&self) -> &BirdResourceRow {
        &self.eggs
    }

    pub fn get_eggs_cap(&self) -> &BirdResourceRow {
        &self.eggs_cap
    }

    pub fn get_cached_food(&self) -> &Vec<BirdResourceRow> {
        &self.cached_food
    }

    pub fn get_tucked_cards(&self) -> &BirdResourceRow {
        &self.tucked_cards
    }

    pub fn play_a_bird(&mut self, bird_card: BirdCard) -> WingResult<()>{
        // Get indexes to insert at
        let birds_idx = self.birds.len();

        // Push and insert values
        self.birds.push(bird_card);
        self.bird_col_idxs.push(birds_idx);
        self.eggs_cap[self.next_col_to_play] = bird_card.egg_capacity();
        // Update which column to play at
        self.next_col_to_play += 1;

        match bird_card {
            BirdCard::CommonBlackbird
                | BirdCard::EuropeanRoller
                | BirdCard::GreyHeron
                | BirdCard::LongTailedTit => {
                    // They are played side-ways. Unless it is the last column
                    if self.bird_col_idxs.len() < 5 {
                        self.bird_col_idxs.push(birds_idx);
                        self.next_col_to_play += 1;
                    }
                }
            _ => {}

        }


        Ok(())
    }
}

#[cfg(test)]
impl MatRow {
    pub fn new_test(
        bird_col_idxs: Vec<usize>,
        next_col_to_play: usize,
        birds: Vec<BirdCard>,
        tucked_cards: BirdResourceRow,
        cached_food: Vec<BirdResourceRow>,
        eggs: BirdResourceRow,
        eggs_cap: BirdResourceRow,
    ) -> Self {
        Self {
            bird_col_idxs,
            next_col_to_play,
            birds,
            tucked_cards,
            cached_food,
            eggs,
            eggs_cap,
        }
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

    pub fn get_columns(&self) -> Vec<[BirdCard; 3]> {
        let bird_cards = self.rows().map(|mt| mt.get_birds());

        let num_columns = bird_cards.iter().map(|row| row.len()).min().unwrap();

        (0..num_columns)
            .map(|col_idx| {
                [bird_cards[0][col_idx], bird_cards[1][col_idx], bird_cards[2][col_idx]]
            })
            .collect()
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
        } + hab_row.get_birds().len() / 2;

        result.extend((0..num_actions).map(|_| hab_action.clone()));

        if hab_row.get_birds().len() % 2 == 1 {
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

    pub fn put_bird_card(&mut self, bird_card: BirdCard, habitat: &Habitat) -> WingResult<Vec<Action>> {
        let row = self.get_row_mut(habitat);
        if row.get_birds().len() >= 5 {
            return Err(WingError::InvalidAction)
        }

        let egg_cost = (row.col_to_play().unwrap() + 1) / 2;

        let egg_cap = bird_card.egg_capacity();
        row.play_a_bird(bird_card)?;
        self.eggs_cap += egg_cap;

        Ok((0..egg_cost as usize).map(|_| Action::DiscardEgg).collect())
    }

    pub fn rows(&self) -> [&MatRow; 3] {
        [&self.forest, &self.grassland, &self.wetland]
    }

    pub fn egg_count(&self) -> u8 {
        self.num_eggs
    }
}

#[cfg(test)]
impl PlayerMat {
    pub fn new_test(
        forest: MatRow,
        grassland: MatRow,
        wetland: MatRow,
        num_eggs: u8,
        eggs_cap: u8,
    ) -> Self {
        Self {
            forest,
            grassland,
            wetland,
            num_eggs,
            eggs_cap,
        }
    }
}
