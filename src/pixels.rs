use std::fmt::Display;

use pyo3::{prelude::*, pyclass::CompareOp, types::PyType};
use ril::Dynamic;

/// Represents a single-bit pixel that represents either a pixel that is on or off.
#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct BitPixel {
    /// bool: Whether the pixel is on.
    #[pyo3(get, set)]
    value: bool,
}

/// Represents an L, or luminance pixel that is stored as only one single number representing how bright, or intense, the pixel is.
///
/// This can be thought of as the “unit channel” as this represents only a single channel in which other pixel types can be composed of.
#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct L {
    /// int: The luminance value of the pixel, between 0 and 255.
    #[pyo3(get, set)]
    value: u8,
}

/// Represents an RGB pixel.
#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct Rgb {
    /// int: The red component of the pixel.
    #[pyo3(get, set)]
    r: u8,
    /// int: The green component of the pixel.
    #[pyo3(get, set)]
    g: u8,
    /// int: The blue component of the pixel.
    #[pyo3(get, set)]
    b: u8,
}

/// Represents an RGBA pixel.
#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct Rgba {
    /// int: The red component of the pixel.
    #[pyo3(get, set)]
    r: u8,
    /// int: The green component of the pixel.
    #[pyo3(get, set)]
    g: u8,
    /// int: The blue component of the pixel.
    #[pyo3(get, set)]
    b: u8,
    /// int: The alpha component of the pixel.
    #[pyo3(get, set)]
    a: u8,
}

/// The user created Pixel type.
#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub struct Pixel {
    pub inner: Dynamic,
}

impl From<Dynamic> for Pixel {
    fn from(inner: Dynamic) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Pixel {
    /// Create a bitpixel.
    ///
    /// Parameters
    /// ----------
    /// value: bool
    ///     Whether the pixel is on.
    #[classmethod]
    #[pyo3(text_signature = "(cls, value)")]
    fn from_bitpixel(_: &PyType, value: bool) -> Self {
        Self {
            inner: Dynamic::BitPixel(ril::BitPixel(value)),
        }
    }

    /// Create a L Pixel.
    ///
    /// Parameters
    /// ----------
    /// value: int
    ///     The luminance value of the pixel, between 0 and 255.
    #[classmethod]
    #[pyo3(text_signature = "(cls, value)")]
    fn from_l(_: &PyType, value: u8) -> Self {
        Self {
            inner: Dynamic::L(ril::L(value)),
        }
    }

    /// Creates a Rgb Pixel
    ///
    /// Parameters
    /// ----------
    /// r: int
    ///     The red component of the pixel.
    /// g: int
    ///     The green component of the pixel.
    /// b: int
    ///     The blue component of the pixel.
    #[classmethod]
    #[pyo3(text_signature = "(cls, r, g, b)")]
    fn from_rgb(_: &PyType, r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: Dynamic::Rgb(ril::Rgb { r, g, b }),
        }
    }

    /// Creates a Rgba Pixel
    ///
    /// Parameters
    /// ----------
    /// r: int
    ///     The red component of the pixel.
    /// g: int
    ///     The green component of the pixel.
    /// b: int
    ///     The blue component of the pixel.
    /// a: int
    ///     The alpha component of the pixel.
    #[classmethod]
    #[pyo3(text_signature = "(cls, r, g, b, a)")]
    fn from_rgba(_: &PyType, r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: Dynamic::Rgba(ril::Rgba { r, g, b, a }),
        }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py);
                if let Ok(other) = other {
                    let val = self == &other;
                    val.into_py(py)
                } else {
                    false.into_py(py)
                }
            }
            CompareOp::Ne => {
                if let Ok(other) = other.extract::<Self>(py) {
                    let val = self != &other;
                    val.into_py(py)
                } else {
                    true.into_py(py)
                }
            }
            _ => py.NotImplemented(),
        }
    }

    fn __repr__(&self) -> String {
        let out = match self.inner {
            Dynamic::BitPixel(v) => format!("BitPixel({})", v.value()),
            Dynamic::L(v) => format!("L({})", v.value()),
            Dynamic::Rgb(v) => format!("Rgb({}, {}, {})", v.r, v.g, v.b),
            Dynamic::Rgba(v) => format!("Rgba({}, {}, {}, {})", v.r, v.g, v.b, v.a),
        };

        format!("<Pixel {}>", out)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.__repr__())
    }
}

#[pymethods]
impl BitPixel {
    #[new]
    fn new(value: bool) -> Self {
        Self { value }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py);
                if let Ok(other) = other {
                    let val = self == &other;
                    val.into_py(py)
                } else {
                    false.into_py(py)
                }
            }
            CompareOp::Ne => {
                if let Ok(other) = other.extract::<Self>(py) {
                    let val = self != &other;
                    val.into_py(py)
                } else {
                    true.into_py(py)
                }
            }
            _ => py.NotImplemented(),
        }
    }

    fn __repr__(&self) -> String {
        format!("<BitPixel value={}>", self.value)
    }
}

#[pymethods]
impl L {
    #[new]
    fn new(value: u8) -> Self {
        Self { value }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py);
                if let Ok(other) = other {
                    let val = self == &other;
                    val.into_py(py)
                } else {
                    false.into_py(py)
                }
            }
            CompareOp::Ne => {
                if let Ok(other) = other.extract::<Self>(py) {
                    let val = self != &other;
                    val.into_py(py)
                } else {
                    true.into_py(py)
                }
            }
            _ => py.NotImplemented(),
        }
    }

    fn __repr__(&self) -> String {
        format!("<L value={}>", self.value)
    }
}

#[pymethods]
impl Rgb {
    #[new]
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py);
                if let Ok(other) = other {
                    let val = self == &other;
                    val.into_py(py)
                } else {
                    false.into_py(py)
                }
            }
            CompareOp::Ne => {
                if let Ok(other) = other.extract::<Self>(py) {
                    let val = self != &other;
                    val.into_py(py)
                } else {
                    true.into_py(py)
                }
            }
            _ => py.NotImplemented(),
        }
    }

    fn __repr__(&self) -> String {
        format!("<Rgb r={} g={} b={}>", self.r, self.g, self.b)
    }
}

#[pymethods]
impl Rgba {
    #[new]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    fn __richcmp__(&self, py: Python<'_>, other: PyObject, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => {
                let other = other.extract::<Self>(py);
                if let Ok(other) = other {
                    let val = self == &other;
                    val.into_py(py)
                } else {
                    false.into_py(py)
                }
            }
            CompareOp::Ne => {
                if let Ok(other) = other.extract::<Self>(py) {
                    let val = self != &other;
                    val.into_py(py)
                } else {
                    true.into_py(py)
                }
            }
            _ => py.NotImplemented(),
        }
    }

    fn __repr__(&self) -> String {
        format!("<Rgba r={} g={} b={} a={}>", self.r, self.g, self.b, self.a)
    }
}

impl From<ril::BitPixel> for BitPixel {
    fn from(pixel: ril::BitPixel) -> Self {
        Self {
            value: pixel.value(),
        }
    }
}

impl From<ril::L> for L {
    fn from(pixel: ril::L) -> Self {
        Self {
            value: pixel.value(),
        }
    }
}

impl From<ril::Rgb> for Rgb {
    fn from(pixel: ril::Rgb) -> Self {
        Self {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
        }
    }
}

impl From<ril::Rgba> for Rgba {
    fn from(pixel: ril::Rgba) -> Self {
        Self {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
            a: pixel.a,
        }
    }
}
