use crate::pixels::{BitPixel, Rgb, Rgba, L};
use pyo3::prelude::*;
use ril::Dynamic;

pub fn cast_pixel_to_pyobject(py: Python<'_>, pixel: Dynamic) -> PyObject {
    match pixel {
        Dynamic::BitPixel(v) => BitPixel::from(v).into_py(py),
        Dynamic::L(v) => L::from(v).into_py(py),
        Dynamic::Rgb(v) => Rgb::from(v).into_py(py),
        Dynamic::Rgba(v) => Rgba::from(v).into_py(py),
    }
}
