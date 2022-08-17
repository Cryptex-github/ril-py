use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

use pyo3::{
    exceptions::{PyIOError, PyRuntimeError, PyTypeError, PyValueError},
    prelude::*,
};
use ril::{Error as RilError, Dynamic};

use crate::workaround::OwnedTextLayout;

pub enum Error {
    Ril(RilError),
    UnexpectedFormat(String, String), // (Expected, Got)
    PoisionError
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Ril(err) => match err {
                RilError::InvalidHexCode(_)
                | RilError::InvalidExtension(_)
                | RilError::UnsupportedColorType => PyValueError::new_err(format!("{}", err)),
                RilError::EncodingError(_)
                | RilError::DecodingError(_)
                | RilError::UnknownEncodingFormat
                | RilError::FontError(_)
                | RilError::IncompatibleImageData { .. } => {
                    PyRuntimeError::new_err(format!("{}", err))
                }
                RilError::IOError(_) => PyIOError::new_err(format!("{}", err)),
                RilError::EmptyImageError => PyRuntimeError::new_err(
                    "Cannot encode an empty image, or an image without data.",
                ),
            },
            Error::UnexpectedFormat(expected, got) => PyTypeError::new_err(format!(
                "Invalid Image format, expected `{}`, got `{}`",
                expected, got
            )),
            Error::PoisionError => PyRuntimeError::new_err("The internal RwLock was poisoned."),
        }
    }
}

impl From<RilError> for Error {
    fn from(err: RilError) -> Self {
        Self::Ril(err)
    }
}

type ReadPoisionError<'a> = PoisonError<RwLockReadGuard<'a, OwnedTextLayout<Dynamic>>>;
type WritePoisionError<'a> = PoisonError<RwLockWriteGuard<'a, OwnedTextLayout<Dynamic>>>;

impl<'a> From<ReadPoisionError<'a>> for Error {
    fn from(_: ReadPoisionError) -> Self {
        Self::PoisionError
    }
}

impl<'a> From<WritePoisionError<'a>> for Error {
    fn from(_: WritePoisionError) -> Self {
        Self::PoisionError
    }
}
