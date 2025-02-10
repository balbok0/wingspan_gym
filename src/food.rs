// Food ids:
// 0 - Invertebrate
// 1 - Seed
// 2 - Fish
// 3 - Fruit
// 4 - Rodent

use strum_macros::EnumIter;

pub type FoodReq = [Option<u8>; 5];
pub type Foods = [u8; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CostAlternative {
    Yes,
    No
}

// Total food cost
pub type BirdCardCost = (FoodReq, u8, CostAlternative);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum FoodIndex {
    Invertebrate = 0,
    Seed = 1,
    Fish = 2,
    Fruit = 3,
    Rodent = 4,
}

impl FoodIndex {
    pub fn dice_sides(&self) -> Vec<u8> {
        match self {
            FoodIndex::Invertebrate => vec![0, 5],
            FoodIndex::Seed => vec![1, 5],
            FoodIndex::Fish => vec![2],
            FoodIndex::Fruit => vec![3],
            FoodIndex::Rodent => vec![4],
        }
    }
}

impl From<u8> for FoodIndex {
    fn from(value: u8) -> Self {
        match value {
            0 => FoodIndex::Invertebrate,
            1 => FoodIndex::Seed,
            2 => FoodIndex::Fish,
            3 => FoodIndex::Fruit,
            4 => FoodIndex::Rodent,
            x => panic!("Got {x} for FoodIndex which is more than max value of 4.")
        }
    }
}

impl From<usize> for FoodIndex {
    fn from(value: usize) -> Self {
        (value as u8).into()
    }
}

impl From<FoodIndex> for u8 {
    fn from(val: FoodIndex) -> Self {
        match val {
            FoodIndex::Invertebrate => 0,
            FoodIndex::Seed => 1,
            FoodIndex::Fish => 2,
            FoodIndex::Fruit => 3,
            FoodIndex::Rodent => 4,
        }
    }
}