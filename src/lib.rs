mod draw;
mod error;
mod image;
mod pixels;
mod sequence;
mod types;
mod utils;

use draw::{Border, Ellipse, Rectangle};
use image::Image;
use pixels::*;
use pyo3::prelude::*;
use sequence::{Frame, ImageSequence};
use types::*;

type Xy = (u32, u32);

macro_rules! add_classes {
    ($m:expr, $($class:ty),*) => {{
        $(
            $m.add_class::<$class>()?;
        )*
    }};
}

#[pymodule]
fn ril(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    add_classes!(
        m,
        BitPixel,
        Image,
        L,
        Pixel,
        Rgb,
        Rgba,
        Border,
        Rectangle,
        DisposalMethod,
        ResizeAlgorithm,
        Frame,
        Ellipse,
        ImageSequence
    );

    Ok(())
}
