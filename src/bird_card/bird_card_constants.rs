use pyo3::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(eq, eq_int)]
pub enum BirdCardColor {
    White = 0,
    Brown = 1,
    Pink = 2,
    None = 3,
    Teal = 4,
    Yellow = 5,
}

impl BirdCardColor {
    pub fn unique_id(&self) -> usize {
        match self {
            BirdCardColor::Brown => 0,
            BirdCardColor::Pink => 1,
            BirdCardColor::Teal => 2,
            BirdCardColor::Yellow => 3,
            // From description: "Birds with no power count as white."
            // For Ethologist & Behaviorist
            BirdCardColor::White | BirdCardColor::None => 4,
        }
    }
}
