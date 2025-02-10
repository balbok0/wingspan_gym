use pyo3::{exceptions::PyValueError, PyErr};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum WingError {
    #[error("Invalid action")]
    InvalidAction,
}

impl From<WingError> for PyErr {
    fn from(val: WingError) -> Self {
        match val {
            WingError::InvalidAction => PyValueError::new_err(format!("{}", val))
        }
    }
}

pub type WingResult<T> = Result<T, WingError>;