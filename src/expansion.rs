#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Expansion {
    Core,
    Asia,
    European,
    Oceania,
}