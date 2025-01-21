pub type ResourceReq = [Option<u8>; 5];
pub type Resources = [u8; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CostAlternative {
    Yes,
    No
}

// Total food cost
pub type BirdCardCost = (ResourceReq, u8, CostAlternative);
