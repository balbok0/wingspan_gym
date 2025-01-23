// Food ids:
// 0 - Invertebrate
// 1 - Seed
// 2 - Fish
// 3 - Fruit
// 4 - Rodent

pub type FoodReq = [Option<u8>; 5];
pub type Foods = [u8; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CostAlternative {
    Yes,
    No
}

// Total food cost
pub type BirdCardCost = (FoodReq, u8, CostAlternative);
