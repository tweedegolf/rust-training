use pyo3::prelude::*;

#[derive(Debug)]
pub enum HelloError {
    Io(std::io::Error),
    AppleStuckInThroat,
}

impl std::fmt::Display for HelloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelloError::Io(e) => write!(f, "Hello Error: {e}"),
            HelloError::AppleStuckInThroat => write!(f, "Hello Error: 🍏🍏"),
        }
    }
}

impl From<HelloError> for PyErr {
    fn from(e: HelloError) -> Self {
        use pyo3::exceptions::*;

        match e {
            HelloError::Io(_) => PyIOError::new_err(e.to_string()),
            HelloError::AppleStuckInThroat => PyBaseException::new_err(e.to_string()),
        }
    }
}

#[pyfunction]
pub fn throws_error() -> PyResult<()> {
    Err(HelloError::AppleStuckInThroat)?;
    Ok(())
}
