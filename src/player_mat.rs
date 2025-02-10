use crate::{action::Action, bird_card::{BirdCard, BirdCardColor}, error::{WingError, WingResult}, food::FoodIndex, habitat::Habitat, wingspan_env::WingspanEnv};

type BirdResourceRow = [u8; 5];


#[derive(Debug, Clone)]
pub struct MatRow {
    // Mapping from column idx -> index in birds. This is because some birds can cover multiple places
    habitat: Habitat,
    bird_col_idxs: Vec<usize>,
    next_col_to_play: usize,
    birds: Vec<BirdCard>,
    tucked_cards: Vec<u8>,
    cached_food: Vec<BirdResourceRow>,
    eggs: Vec<u8>,
    eggs_cap: Vec<u8>,
}

impl MatRow {
    pub fn new(habitat: Habitat) -> Self {
        Self {
            habitat,
            birds: Vec::with_capacity(5),
            bird_col_idxs: Vec::with_capacity(5),
            next_col_to_play: 0,
            tucked_cards: Vec::with_capacity(5),
            cached_food: Vec::with_capacity(5),
            eggs: Vec::with_capacity(5),
            eggs_cap: Vec::with_capacity(5),
        }
    }

    pub fn col_to_play(&self) -> Option<u8> {
        if self.next_col_to_play >= 5 {
            None
        } else {
            Some(self.next_col_to_play as u8)
        }
    }

    pub fn bird_at_idx(&self, idx: usize) -> Option<BirdCard> {
        Some(*self.birds.get(*self.bird_col_idxs.get(idx)?)?)
    }


    pub fn get_bird_actions(&self, _env: &mut WingspanEnv) -> (Vec<Action>, Vec<Action>) {
        let actions = vec![];
        let end_of_turn_actions = vec![];

        // Iterate through birds from right to left
        for (_bird_idx, bird) in self.birds.iter().enumerate().rev() {
            if bird.color() != &BirdCardColor::Brown {
                continue
            }

            // TODO: After all (from core) of the BirdCard actions are implemented, do uncomment below
            // if let Ok(mut action_res) =  bird.activate(env, &self.habitat, bird_idx) {
            //     actions.append(&mut action_res.immediate_actions);
            //     end_of_turn_actions.append(&mut action_res.end_of_turn_actions);
            // }
        }

        // Actions are pushed onto back of the queue, so reverse to match order of actions
        (actions.into_iter().rev().collect(), end_of_turn_actions)
    }

    pub fn num_spots_to_place_eggs(&self) -> usize {
        self.eggs.iter()
            .zip(self.eggs_cap.iter())
            .filter(|(eggs, cap)| eggs < cap)
            .count()
    }

    pub fn num_spots_to_discard_eggs(&self) -> usize {
        self.eggs.iter()
            .filter(|eggs| **eggs > 0)
            .count()
    }

    pub fn place_egg(&mut self, idx: usize) -> Result<(), usize> {
        let mut count = 0;

        for (col_idx, (egg, cap)) in self.eggs.iter().zip(self.eggs_cap.iter()).enumerate() {
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
    pub fn place_egg_at_exact_column(&mut self, col_idx: usize) -> WingResult<()> {
        let bird_idx = match self.bird_col_idxs.get(col_idx) {
            Some(bird_idx) => *bird_idx,
            None => return Err(WingError::InvalidAction),
        };

        self.place_egg_at_exact_bird_idx(bird_idx);
        Ok(())
    }

    pub fn place_egg_at_exact_bird_idx(&mut self, bird_idx: usize) {
        if self.eggs_cap[bird_idx] > self.eggs[bird_idx] {
            self.eggs[bird_idx] += 1;
        }
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

    pub fn get_eggs(&self) -> &Vec<u8> {
        &self.eggs
    }

    pub fn get_eggs_cap(&self) -> &Vec<u8> {
        &self.eggs_cap
    }

    pub fn get_cached_food(&self) -> &Vec<BirdResourceRow> {
        &self.cached_food
    }

    pub fn get_tucked_cards(&self) -> &Vec<u8> {
        &self.tucked_cards
    }

    pub fn play_a_bird(&mut self, bird_card: BirdCard) -> WingResult<()>{
        // Get indexes to insert at
        let birds_idx = self.birds.len();

        // Push and insert values
        self.birds.push(bird_card);
        self.bird_col_idxs.push(birds_idx);
        self.cached_food.push(Default::default());
        self.tucked_cards.push(0);
        self.eggs.push(0);
        self.eggs_cap.push(bird_card.egg_capacity());
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

    pub fn cache_food(&mut self, bird_idx: usize, food_idx: FoodIndex) {
        self.cached_food[bird_idx][food_idx as usize] += 1;
    }

    pub fn tuck_card(&mut self, bird_idx: usize) {
        self.tucked_cards[bird_idx] += 1;
    }

    pub fn add_bird_from_entry(
        &mut self,
        bird_card: BirdCard,
        tucked_cards: u8,
        cached_food: BirdResourceRow,
        eggs: u8,
        eggs_cap: u8,
    ) -> WingResult<()> {
        let new_bird_idx = self.birds.len();
        self.play_a_bird(bird_card)?;
        self.tucked_cards[new_bird_idx] = tucked_cards;
        self.cached_food[new_bird_idx] = cached_food;
        self.eggs[new_bird_idx] = eggs;
        self.eggs_cap[new_bird_idx] = eggs_cap;
        Ok(())
    }

    pub fn remove_bird(&mut self, bird_idx: usize) -> (BirdCard, u8, BirdResourceRow, u8, u8) {
        let result = (
            self.birds.remove(bird_idx),
            self.tucked_cards.remove(bird_idx),
            self.cached_food.remove(bird_idx),
            self.eggs.remove(bird_idx),
            self.eggs_cap.remove(bird_idx),
        );
        self.next_col_to_play -= 1;
        result
    }
}

#[cfg(test)]
impl MatRow {
    pub fn new_test(
        habitat: Habitat,
        bird_col_idxs: Vec<usize>,
        next_col_to_play: usize,
        birds: Vec<BirdCard>,
        tucked_cards: Vec<u8>,
        cached_food: Vec<BirdResourceRow>,
        eggs: Vec<u8>,
        eggs_cap: Vec<u8>,
    ) -> Self {
        Self {
            habitat,
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
}

impl Default for PlayerMat {
    fn default() -> Self {
        Self {
            forest: MatRow::new(Habitat::Forest),
            grassland: MatRow::new(Habitat::Grassland),
            wetland: MatRow::new(Habitat::Wetland),
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

    pub fn get_row_mut(&mut self, habitat: &Habitat) -> &mut MatRow {
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

    pub fn get_columns(&self) -> Vec<[Option<&BirdCard>; 3]> {
        let bird_cards = self.rows().map(|mt| mt.get_birds());

        let num_columns = bird_cards.iter().map(|row| row.len()).max().unwrap();

        (0..num_columns)
            .map(|col_idx| {
                [bird_cards[0].get(col_idx), bird_cards[1].get(col_idx), bird_cards[2].get(col_idx)]
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

                if egg_req > self.num_eggs() {
                    // Not enough eggs
                    return false;
                }

                // Eggs are satisfied and there is a place
                true
            } else {
                // No place in a habitat
                false
            }
        }).cloned().collect()
    }

    pub fn get_actions_from_habitat_action(&self, habitat: &Habitat) -> Vec<Action> {
        let hab_action = habitat.action();
        let hab_row = self.get_row(habitat);

        let mut result = vec![Action::BirdActionFromHabitat(*habitat)];

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
        self.rows().map(MatRow::num_spots_to_place_eggs).iter().sum()
    }

    pub fn num_spots_to_discard_eggs(&self) -> usize {
        self.rows().map(MatRow::num_spots_to_discard_eggs).iter().sum()
    }

    pub fn move_bird(&mut self, bird_card: BirdCard, target_habitat: Habitat) -> WingResult<()> {
        if self.get_row(&target_habitat).col_to_play().is_none() {
            return Err(WingError::InvalidAction);
        }

        let (source_habitat, bird_idx) = self.find_bird(&bird_card).ok_or(WingError::InvalidAction)?;

        let (bird_card, tucked_cards, cached_food, eggs, eggs_cap) = self.get_row_mut(&source_habitat).remove_bird(bird_idx);

        self.get_row_mut(&target_habitat).add_bird_from_entry(bird_card, tucked_cards, cached_food, eggs, eggs_cap)?;

        Ok(())
    }

    fn find_bird(&self, bird_card: &BirdCard) -> Option<(Habitat, usize)> {
        for row in self.rows() {
            if let Some(bird_idx) = row.birds.iter().position(|bc| bc == bird_card) {
                return Some((row.habitat, bird_idx))
            }
        }
        None
    }

    pub fn place_egg(&mut self, idx: u8) -> WingResult<()> {
        let idx = idx as usize;
        let mut cur_action_count = 0;
        for hab_row in [&mut self.forest, &mut self.grassland, &mut self.wetland] {
            match hab_row.place_egg(idx - cur_action_count) {
                Ok(()) => return Ok(()),
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
                Ok(()) => return Ok(()),
                Err(num_actions_in_row) => {
                    cur_action_count += num_actions_in_row;
                }
            }
        }

        // No places found to place eggs, so this was an invalid action
        Err(WingError::InvalidAction)
    }

    pub fn num_eggs(&self) -> u8 {
        self.rows().iter().flat_map(|mat_row| mat_row.eggs.iter()).sum()
    }

    pub fn eggs_cap(&self) -> u8 {
        self.rows().iter().flat_map(|mat_row| mat_row.eggs_cap.iter()).sum()
    }

    pub fn can_place_egg(&self) -> bool {
        self.num_eggs() < self.eggs_cap()
    }

    pub fn can_discard_egg(&self) -> bool {
        self.num_eggs() > 0
    }

    pub fn put_bird_card(&mut self, bird_card: BirdCard, habitat: &Habitat) -> WingResult<Vec<Action>> {
        let row = self.get_row_mut(habitat);
        if row.get_birds().len() >= 5 {
            return Err(WingError::InvalidAction)
        }

        let egg_cost = (row.col_to_play().unwrap() + 1) / 2;

        row.play_a_bird(bird_card)?;

        Ok((0..egg_cost as usize).map(|_| Action::DiscardEgg).collect())
    }

    pub fn rows(&self) -> [&MatRow; 3] {
        [&self.forest, &self.grassland, &self.wetland]
    }

    pub fn egg_count(&self) -> u8 {
        self.num_eggs()
    }
}

#[cfg(test)]
impl PlayerMat {
    pub fn new_test(
        forest: MatRow,
        grassland: MatRow,
        wetland: MatRow,
    ) -> Self {
        Self {
            forest,
            grassland,
            wetland,
        }
    }
}
