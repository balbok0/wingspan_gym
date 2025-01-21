use pyo3::{exceptions::PyValueError, PyErr};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum WingError {
    #[error("Invalid action")]
    InvalidAction,
}

impl Into<PyErr> for WingError {
    fn into(self) -> PyErr {
        match self {
            WingError::InvalidAction => PyValueError::new_err(format!("{}", self))
        }
    }
}

pub type WingResult<T> = Result<T, WingError>;