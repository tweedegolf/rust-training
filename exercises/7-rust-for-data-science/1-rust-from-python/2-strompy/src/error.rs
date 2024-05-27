use std::{
    fmt::{write, Display},
    num::{ParseFloatError, ParseIntError},
};

use pyo3::{
    exceptions::{PyException, PyValueError},
    prelude::*,
};

#[derive(Debug)]
/// Strompy's error type. Encapsulates everything that can go wrong
/// when working with Strompy
pub enum StrompyError {
    Json(&'static str),
    Struson(struson::reader::ReaderError),
    ParseFloat(ParseFloatError),
    ParseInt(std::num::ParseIntError),
    InvalidDimensions,
}

impl Display for StrompyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrompyError::Json(e) => write!(f, r#"Unexpected key encountered, expected "{e}""#),
            StrompyError::Struson(e) => write!(f, "Struson error: {e}"),
            StrompyError::ParseFloat(e) => write!(f, "ParseFloat error: {e}"),
            StrompyError::ParseInt(e) => write!(f, "ParseInt error: {e}"),
            StrompyError::InvalidDimensions => {
                write!(f, "Invalid matrix dimensions to perform that operation")
            }
        }
    }
}

impl From<StrompyError> for pyo3::PyErr {
    fn from(e: StrompyError) -> Self {
        match e {
            StrompyError::ParseFloat(e) => PyValueError::new_err(e.to_string()),
            StrompyError::ParseInt(e) => PyValueError::new_err(e.to_string()),
            e => PyException::new_err(e.to_string()),
        }
    }
}

impl From<struson::reader::ReaderError> for StrompyError {
    fn from(e: struson::reader::ReaderError) -> Self {
        Self::Struson(e)
    }
}

impl From<ParseFloatError> for StrompyError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}

impl From<ParseIntError> for StrompyError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}
