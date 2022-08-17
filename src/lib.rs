#![allow(clippy::missing_const_for_fn)]
// Warnings created by pyo3
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::borrow_deref_ref)]
#![allow(clippy::use_self)]

mod draw;
mod error;
mod image;
mod pixels;
mod sequence;
mod types;
mod utils;
mod text;
mod workaround;

use draw::{Border, Ellipse, Rectangle};
use image::Image;
use pixels::{BitPixel, Pixel, Rgb, Rgba, L};
use pyo3::prelude::*;
use sequence::{Frame, ImageSequence};
use types::{DisposalMethod, ResizeAlgorithm};

use text::{TextLayout, TextSegment, Font};

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
        ImageSequence,
        TextSegment,
        TextLayout,
        Font
    );

    Ok(())
}
