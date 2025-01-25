#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoringRule {
    Each(u8),
    Ladder(Box<[(u8, u8)]>),
}
