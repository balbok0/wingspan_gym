use pyo3::{exceptions::PyValueError, PyErr};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum WingError {
    #[error("Invalid action")]
    InvalidAction,

    #[error("Invalid bird")]
    InvalidBird(String),
}

impl From<WingError> for PyErr {
    fn from(val: WingError) -> Self {
        match val {
            WingError::InvalidAction => PyValueError::new_err(format!("{}", val)),
            WingError::InvalidBird(err_msg) => PyValueError::new_err(format!("{}", err_msg)),
        }
    }
}

pub type WingResult<T> = Result<T, WingError>;