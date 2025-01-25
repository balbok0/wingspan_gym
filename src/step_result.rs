use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub enum StepResult {
    Live,
    Terminated,
}
