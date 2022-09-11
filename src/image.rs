use std::path::PathBuf;

use crate::draw::DrawEntity;
use crate::error::Error;
use crate::pixels::{BitPixel, Pixel, Rgb, Rgba, L};
use crate::types::{ResizeAlgorithm, OverlayMode};
use crate::utils::cast_pixel_to_pyobject;
use pyo3::types::PyBytes;
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyTuple, PyType},
};
use ril::{Banded, Dynamic, Image as RilImage, ImageFormat};

/// A high-level image representation.
///
/// This represents a static, single-frame image. See :class:`.ImageSequence` for information on opening animated or multi-frame images.
#[pyclass]
#[derive(Clone)]
pub struct Image {
    pub inner: RilImage<Dynamic>,
}

macro_rules! cast_bands_to_pyobjects {
    ($py:expr, $($band:expr),*) => {{
        Ok((
            $(
                Self::from_inner($band.convert::<Dynamic>()),
            )*
        ).into_py($py))
    }};
}

macro_rules! to_inner_bands {
    ($bands:expr, $($band:tt),*) => {{
        (
            $(
                $bands.$band.inner.convert::<ril::L>(),
            )*
        )
    }};
}

macro_rules! ensure_mode {
    ($bands:expr, $($band:tt),*) => {{
        $(
            if $bands.$band.mode() != "L" {
                return Err(PyTypeError::new_err(format!("Expected mode `L`, got `{}`", $bands.$band.mode())));
            }
        )*

        Ok::<(), PyErr>(())
    }};
}

#[pymethods]
impl Image {
    /// Creates a new image with the given width and height, with all pixels being set intially to `fill`.
    ///
    /// Parameters
    /// ----------
    /// width: int
    ///     The width of the Image.
    /// height: int
    ///     The height of the Image.
    /// fill: :class:`.Pixel`
    ///     The pixel used to fill the image.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     Image.new(100, 100, Pixel.from_rgb(255, 255, 255))
    #[classmethod]
    #[pyo3(text_signature = "(cls, width, height, fill)")]
    fn new(_: &PyType, width: u32, height: u32, fill: Pixel) -> Self {
        Self {
            inner: RilImage::new(width, height, fill.inner),
        }
    }

    /// Decodes an image with the explicitly given image encoding from the raw bytes.
    ///
    /// if `format` is not provided then it will try to infer its encoding.
    ///
    /// Parameters
    /// ----------
    /// bytes: bytes
    ///     The bytes of the Image.
    /// format: Optional[str], default: None
    ///     The format of the image, defaults to `None`.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     Raised if the format provided is invalid.
    /// RuntimeError
    ///     Raised if the image can't be decoded or the format is unknown.
    #[classmethod]
    #[pyo3(text_signature = "(cls, bytes, format = None)")]
    fn from_bytes(_: &PyType, bytes: &[u8], format: Option<&str>) -> Result<Self, Error> {
        Ok(if let Some(format) = format {
            Self {
                inner: RilImage::from_bytes(ImageFormat::from_extension(format)?, bytes)?,
            }
        } else {
            Self {
                inner: RilImage::from_bytes_inferred(bytes)?,
            }
        })
    }

    /// Creates a new image shaped with the given width
    /// and a 1-dimensional sequence of pixels which will be shaped according to the width.
    ///
    /// Parameters
    /// ----------
    /// width: int
    ///     The width of the image.
    /// pixels: List[:class:`.Pixel`]
    ///     A List of pixels.
    #[classmethod]
    #[pyo3(text_signature = "(cls, width, pixels)")]
    fn from_pixels(_: &PyType, width: u32, pixels: Vec<Pixel>) -> Self {
        Self {
            inner: RilImage::from_pixels(
                width,
                pixels
                    .into_iter()
                    .map(|p| p.inner)
                    .collect::<Vec<Dynamic>>(),
            ),
        }
    }

    /// Opens a file from the given path and decodes it into an image.
    ///
    /// The encoding of the image is automatically inferred.
    /// You can explicitly pass in an encoding by using the :meth:`from_bytes` method.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the image.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The file extension is invalid.
    /// RuntimeError
    ///     Failed to infer file format or Failed to decode image.
    #[classmethod]
    #[pyo3(text_signature = "(cls, path)")]
    fn open(_: &PyType, path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            inner: RilImage::open(path)?,
        })
    }

    /// :class:`.OverlayMode`: Returns the overlay mode of the image.
    #[getter]
    fn overlay_mode(&self) -> OverlayMode {
        self.inner.overlay_mode().into()
    }

    /// str: Returns the mode of the image.
    #[getter]
    fn mode(&self) -> &str {
        match self.inner.pixel(0, 0) {
            Dynamic::BitPixel(_) => "bitpixel",
            Dynamic::L(_) => "L",
            Dynamic::Rgb(_) => "RGB",
            Dynamic::Rgba(_) => "RGBA",
        }
    }

    /// int: Returns the width of the image.
    #[getter]
    fn width(&self) -> u32 {
        self.inner.width()
    }

    /// int: Returns the height of the image.
    #[getter]
    fn height(&self) -> u32 {
        self.inner.height()
    }

    /// Return the bands of the image.
    ///
    /// Returns
    /// -------
    /// Tuple[:class:`.L`, ...]
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     The image is not of mode `RGB` or `RGBA`.
    fn bands(&self, py: Python<'_>) -> Result<PyObject, Error> {
        match self.mode() {
            "RGB" => {
                let (r, g, b) = self.inner.clone().convert::<ril::Rgb>().bands();

                cast_bands_to_pyobjects!(py, r, g, b)
            }
            "RGBA" => {
                let (r, g, b, a) = self.inner.clone().convert::<ril::Rgba>().bands();

                cast_bands_to_pyobjects!(py, r, g, b, a)
            }
            _ => Err(Error::UnexpectedFormat(
                self.mode().to_string(),
                "Rgb or Rgba".to_string(),
            )),
        }
    }

    /// Creates a new image from the given bands.
    ///
    /// Parameters
    /// ----------
    /// bands: \* :class:`.L`
    ///     The bands of the image.
    #[classmethod]
    #[args(bands = "*")]
    #[pyo3(text_signature = "(self, *bands)")]
    fn from_bands(_: &PyType, bands: &PyTuple) -> PyResult<Self> {
        match bands.len() {
            3 => {
                let bands: (Self, Self, Self) = bands.extract()?;

                ensure_mode!(bands, 0, 1, 2)?;

                Ok(Self::from_inner(
                    RilImage::from_bands(to_inner_bands!(bands, 0, 1, 2)).convert::<ril::Dynamic>(),
                ))
            }
            4 => {
                let bands: (Self, Self, Self, Self) = bands.extract()?;

                ensure_mode!(bands, 0, 1, 2, 3)?;

                Ok(Self::from_inner(
                    RilImage::from_bands(to_inner_bands!(bands, 0, 1, 2, 3))
                        .convert::<ril::Dynamic>(),
                ))
            }
            _ => Err(PyValueError::new_err(format!(
                "Expected 3 or 4 arguments, got `{}`",
                bands.len()
            ))),
        }
    }

    /// Crops this image in place to the given bounding box.
    ///
    /// Parameters
    /// ----------
    /// x1: int
    ///    The x axis of the upper-left corner
    /// y1: int
    ///     The y axis of the upper-left corner
    /// x2: int
    ///     The x axis of the lower-right corner
    /// y2: int
    ///     The y axis of the lower-right corner
    #[pyo3(text_signature = "(self, x1, y1, x2, y2)")]
    fn crop(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.inner.crop(x1, y1, x2, y2);
    }

    /// Draws an object or shape onto this image.
    ///
    /// Parameters
    /// ----------
    /// entity: Union[:class:`.Rectangle`, :class:`.Ellipse`]
    ///     The entity to draw on the image.
    #[pyo3(text_signature = "(self, entity)")]
    fn draw(&mut self, entity: DrawEntity) {
        entity.0.draw(&mut self.inner);
    }

    /// Resizes this image in place to the given dimensions using the given resizing algorithm in place.
    ///
    /// Parameters
    /// ----------
    /// width: int
    ///     The target width to resize to
    /// height: int
    ///     The target height to resize to
    /// algorithm: :class:`.ResizeAlgorithm`
    ///     The resize algorithm to use
    #[pyo3(text_signature = "(self, width, height, algorithm)")]
    fn resize(&mut self, width: u32, height: u32, algorithm: ResizeAlgorithm) {
        self.inner.resize(width, height, algorithm.into());
    }

    /// Encodes the image with the given encoding and returns `bytes`.
    ///
    /// Parameters
    /// ----------
    /// encoding: str
    ///     The encoding of the image.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     The encoded bytes of the image.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The encoding is invalid.
    /// RuntimeError
    ///     Failed to encode the image.
    #[pyo3(text_signature = "(self, encoding)")]
    fn encode(&self, encoding: &str) -> Result<&PyBytes, Error> {
        let encoding = ImageFormat::from_extension(encoding)?;

        let mut buf = Vec::new();
        self.inner.encode(encoding, &mut buf)?;

        // SAFETY: We acquired the GIL before calling `assume_gil_acquired`.
        // `assume_gil_acquired` is only used to ensure that PyBytes don't outlive the current function
        unsafe {
            Python::with_gil(|_| {
                let buf = buf.as_slice();
                let pyacq = Python::assume_gil_acquired();
                Ok(PyBytes::new(pyacq, buf))
            })
        }
    }

    /// Saves the image to the given path.
    /// If encoding is not provided, it will attempt to infer it by the path/filename's extension
    /// You can try saving to a memory buffer by using the :meth:`encode` method.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to save the image to.
    /// encoding: Optional[str], default: None
    ///     The encoding of the image, defaults to `None`.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The encoding provided is invalid.
    /// RuntimeError
    ///     Failed to encode the image or Failed to infer the image format.
    #[pyo3(text_signature = "(self, path, encoding = None)")]
    fn save(&self, path: PathBuf, encoding: Option<&str>) -> Result<(), Error> {
        if let Some(encoding) = encoding {
            let encoding = ImageFormat::from_extension(encoding)?;
            self.inner.save(encoding, path)?;
        } else {
            self.inner.save_inferred(path)?;
        }

        Ok(())
    }

    /// Returns a 2D list representing the pixels of the image. Each list in the list is a row.
    ///
    /// For example:
    ///
    /// [[Pixel, Pixel, Pixel], [Pixel, Pixel, Pixel]]
    ///
    /// where the width of the inner list is determined by the width of the image.
    ///
    /// .. warning:: **This function involves heavy operation**
    ///
    ///     This function requires multiple iterations, so it is a heavy operation for larger image.
    ///
    /// Returns
    /// -------
    /// List[List[Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]]]
    ///     The pixels of the image.
    fn pixels(&self, py: Python<'_>) -> Vec<Vec<PyObject>> {
        self.inner
            .pixels()
            .into_iter()
            .map(|p| {
                p.into_iter()
                    .map(|p| cast_pixel_to_pyobject(py, p.clone()))
                    .collect::<Vec<PyObject>>()
            })
            .collect::<Vec<Vec<PyObject>>>()
    }

    /// Pastes the given image onto this image at the given x and y axis.
    ///
    /// If `mask` is provided it will be masked with the given masking image.
    ///
    /// Currently, only BitPixel images are supported for the masking image.
    ///
    /// Parameters
    /// ----------
    /// x: int
    ///     The x axis
    /// y: int
    ///     The y axis
    /// image: :class:`Image`
    ///     The image to paste.
    /// mask: Optional[:class:`Image`], default: None
    ///     The mask to use, defaults to `None`
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The mask provided is not of mode `BitPixel`
    #[pyo3(text_signature = "(self, x, y, image, mask = None)")]
    fn paste(&mut self, x: u32, y: u32, image: Self, mask: Option<Self>) -> Result<(), Error> {
        if let Some(mask) = mask {
            if mask.mode() != "bitpixel" {
                return Err(Error::UnexpectedFormat(
                    "bitpixel".to_string(),
                    mask.mode().to_string(),
                ));
            }

            self.inner
                .paste_with_mask(x, y, image.inner, mask.inner.convert::<ril::BitPixel>());
        } else {
            self.inner.paste(x, y, image.inner);
        }

        Ok(())
    }

    /// Masks the alpha values of this image with the luminance values of the given single-channel L image.
    ///
    /// If you want to mask using the alpha values of the image instead of providing an L image, you can split the bands of the image and extract the alpha band.
    ///
    /// This masking image must have the same dimensions as this image.
    ///
    /// Parameters
    /// ----------
    /// mask: :class:`Image`
    ///     The mask to use
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The mask provided is not of mode `L`
    #[pyo3(text_signature = "(self, mask)")]
    fn mask_alpha(&mut self, mask: Self) -> Result<(), Error> {
        if mask.mode() != "L" {
            return Err(Error::UnexpectedFormat(
                "L".to_string(),
                mask.mode().to_string(),
            ));
        }

        self.inner.mask_alpha(&mask.inner.convert::<ril::L>());

        Ok(())
    }

    /// Mirrors, or flips this image horizontally (about the y-axis) in place.
    fn mirror(&mut self) {
        self.inner.mirror();
    }

    /// Flips this image vertically (about the x-axis) in place.
    fn flip(&mut self) {
        self.inner.flip();
    }

    /// str: Returns the encoding format of the image.
    ///
    /// .. note::
    ///     This is nothing more but metadata about the image.
    ///     When saving the image, you will still have to explicitly specify the encoding format.
    #[getter]
    fn format(&self) -> String {
        format!("{}", self.inner.format())
    }

    /// Tuple[int, int]: Returns the dimensions of the image.
    #[getter]
    fn dimensions(&self) -> (u32, u32) {
        self.inner.dimensions()
    }

    /// Returns the pixel at the given coordinates.
    ///
    /// Parameters
    /// ----------
    /// x: int
    ///     The x axis
    /// y: int
    ///     The y axis
    ///
    /// Returns
    /// -------
    /// Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]
    ///     The pixel of that specific coordinate.
    #[pyo3(text_signature = "(self, x, y)")]
    fn get_pixel(&self, py: Python<'_>, x: u32, y: u32) -> PyObject {
        match *self.inner.pixel(x, y) {
            Dynamic::BitPixel(v) => BitPixel::from(v).into_py(py),
            Dynamic::L(v) => L::from(v).into_py(py),
            Dynamic::Rgb(v) => Rgb::from(v).into_py(py),
            Dynamic::Rgba(v) => Rgba::from(v).into_py(py),
        }
    }

    /// Sets the pixel at the given coordinates to the given pixel.
    ///
    /// Parameters
    /// ---------
    /// x: int
    ///     The x axis
    /// y: int
    ///     The y axis
    /// pixel: :class:`.Pixel`
    ///     The pixel to set it to
    #[pyo3(text_signature = "(self, x, y, pixel)")]
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.inner.set_pixel(x, y, pixel.inner)
    }

    /// Inverts the image in-place.
    fn invert(&mut self) {
        self.inner.invert();
    }

    fn __len__(&self) -> usize {
        self.inner.len() as usize
    }

    fn __repr__(&self) -> String {
        format!(
            "<Image mode={} width={} height={} format={} dimensions=({}, {})>",
            self.mode(),
            self.width(),
            self.height(),
            self.format(),
            self.dimensions().0,
            self.dimensions().1
        )
    }

    fn __bool__(&self) -> bool {
        !self.inner.is_empty()
    }
}

impl Image {
    fn from_inner(image: RilImage) -> Self {
        Self { inner: image }
    }
}
