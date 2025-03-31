use pyo3::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(eq, eq_int)]
pub enum NestType {
    Platform,
    Cavity,
    Wild,
    None,
    Bowl,
    Ground,
}
