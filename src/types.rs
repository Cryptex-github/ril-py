use std::fmt::Display;

use pyo3::prelude::*;

macro_rules! cast_enum {
    ($from:ty, $to:ty, $item:expr, $($var:tt),*) => {{
        match $item {
            $(
                <$from>::$var => <$to>::$var,
            )*
        }
    }};
}

/// A filtering algorithm that is used to resize an image.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[pyclass]
pub enum ResizeAlgorithm {
    /// A simple nearest neighbor algorithm. Although the fastest, this gives the lowest quality
    /// resizings.
    ///
    /// When upscaling this is good if you want a "pixelated" effect with no aliasing.
    Nearest,
    /// A box filter algorithm. Equivalent to the [`Nearest`] filter if you are upscaling.
    Box,
    /// A bilinear filter. Calculates output pixel value using linear interpolation on all pixels.
    Bilinear,
    /// While having similar performance as the [`Bilinear`] filter, this produces a sharper and
    /// usually considered better quality image than the [`Bilinear`] filter, but **only** when
    /// downscaling. This may give worse results than bilinear when upscaling.
    Hamming,
    /// A Catmull-Rom bicubic filter, which is the most common bicubic filtering algorithm. Just
    /// like all cubic filters, it uses cubic interpolation on all pixels to calculate output
    /// pixels.
    Bicubic,
    /// A Mitchell-Netravali bicubic filter. Just like all cubic filters, it uses cubic
    /// interpolation on all pixels to calculate output pixels.
    Mitchell,
    /// A Lanczos filter with a window of 3. Calculates output pixel value using a high-quality
    /// Lanczos filter on all pixels.
    Lanczos3,
}

impl From<ResizeAlgorithm> for ril::ResizeAlgorithm {
    fn from(algo: ResizeAlgorithm) -> ril::ResizeAlgorithm {
        cast_enum!(
            ResizeAlgorithm,
            Self,
            algo,
            Nearest,
            Box,
            Bilinear,
            Hamming,
            Bicubic,
            Mitchell,
            Lanczos3
        )
    }
}

impl From<ril::ResizeAlgorithm> for ResizeAlgorithm {
    fn from(algo: ril::ResizeAlgorithm) -> ResizeAlgorithm {
        cast_enum!(
            ril::ResizeAlgorithm,
            Self,
            algo,
            Nearest,
            Box,
            Bilinear,
            Hamming,
            Bicubic,
            Mitchell,
            Lanczos3
        )
    }
}

/// The method used to dispose a frame before transitioning to the next frame in an image sequence.
#[pyclass]
#[derive(Clone)]
pub enum DisposalMethod {
    /// Do not dispose the current frame. Usually not desired for transparent images.
    Keep,
    /// Dispose the current frame completely and replace it with the image’s background color.
    Background,
    /// Dispose and replace the current frame with the previous frame.
    Previous,
}

impl Display for DisposalMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keep => f.write_str("Keep"),
            Self::Background => f.write_str("Background"),
            Self::Previous => f.write_str("Previous"),
        }
    }
}

impl From<DisposalMethod> for ril::DisposalMethod {
    fn from(method: DisposalMethod) -> ril::DisposalMethod {
        match method {
            DisposalMethod::Keep => ril::DisposalMethod::None,
            DisposalMethod::Background => ril::DisposalMethod::Background,
            DisposalMethod::Previous => ril::DisposalMethod::Previous,
        }
    }
}

impl From<ril::DisposalMethod> for DisposalMethod {
    fn from(method: ril::DisposalMethod) -> Self {
        match method {
            ril::DisposalMethod::None => DisposalMethod::Keep,
            ril::DisposalMethod::Background => DisposalMethod::Background,
            ril::DisposalMethod::Previous => DisposalMethod::Previous,
        }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub enum WrapStyle {
    NoWrap,
    Word,
    Character,
}

impl From<WrapStyle> for ril::WrapStyle {
    fn from(style: WrapStyle) -> Self {
        match style {
            WrapStyle::NoWrap => ril::WrapStyle::None,
            WrapStyle::Word => ril::WrapStyle::Word,
            WrapStyle::Character => ril::WrapStyle::Character,
        }
    }
}

impl From<ril::WrapStyle> for WrapStyle {
    fn from(style: ril::WrapStyle) -> Self {
        match style {
            ril::WrapStyle::None => WrapStyle::NoWrap,
            ril::WrapStyle::Word => WrapStyle::Word,
            ril::WrapStyle::Character => WrapStyle::Character,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub enum OverlayMode {
    Replace,
    Merge,
}

impl From<OverlayMode> for ril::OverlayMode {
    fn from(mode: OverlayMode) -> Self {
        match mode {
            OverlayMode::Replace => ril::OverlayMode::Replace,
            OverlayMode::Merge => ril::OverlayMode::Merge,
        }
    }
}

impl From<ril::OverlayMode> for OverlayMode {
    fn from(mode: ril::OverlayMode) -> Self {
        match mode {
            ril::OverlayMode::Replace => OverlayMode::Replace,
            ril::OverlayMode::Merge => OverlayMode::Merge,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum HorizontalAnchor {
    Left,
    Center,
    Right,
}

impl From<HorizontalAnchor> for ril::HorizontalAnchor {
    fn from(anchor: HorizontalAnchor) -> Self {
        cast_enum!(HorizontalAnchor, Self, anchor, Left, Center, Right)
    }
}

impl From<ril::HorizontalAnchor> for HorizontalAnchor {
    fn from(anchor: ril::HorizontalAnchor) -> Self {
        cast_enum!(ril::HorizontalAnchor, Self, anchor, Left, Center, Right)
    }
}

#[pyclass]
#[derive(Clone)]
pub enum VerticalAnchor {
    Top,
    Center,
    Bottom,
}

impl From<VerticalAnchor> for ril::VerticalAnchor {
    fn from(anchor: VerticalAnchor) -> Self {
        cast_enum!(VerticalAnchor, Self, anchor, Top, Center, Bottom)
    }
}

impl From<ril::VerticalAnchor> for VerticalAnchor {
    fn from(anchor: ril::VerticalAnchor) -> Self {
        cast_enum!(ril::VerticalAnchor, Self, anchor, Top, Center, Bottom)
    }
}
