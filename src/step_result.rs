use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StepResult {
    Live,
    Terminated,
}
