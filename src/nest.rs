use pyo3::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(eq, eq_int)]
pub enum NestType {
    Platform = 0,
    Cavity = 1,
    Wild = 2,
    None = 3,
    Bowl = 4,
    Ground = 5,
}
