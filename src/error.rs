use pyo3::{
    exceptions::{PyIOError, PyRuntimeError, PyTypeError, PyValueError},
    prelude::*,
};
use ril::Error as RilError;

pub enum Error {
    Ril(RilError),
    UnexpectedFormat(String, String), // (Expected, Got)
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
        }
    }
}

impl From<RilError> for Error {
    fn from(err: RilError) -> Self {
        Self::Ril(err)
    }
}
