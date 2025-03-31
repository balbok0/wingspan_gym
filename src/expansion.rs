use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expansion {
    Core = 0,
    Asia = 1,
    European = 2,
    Oceania = 3,
}
