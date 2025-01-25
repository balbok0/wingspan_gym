use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expansion {
    Core,
    Asia,
    European,
    Oceania,
}