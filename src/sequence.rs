use std::{path::PathBuf, time::Duration};

use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};
use ril::{
    Dynamic, Frame as RilFrame, FrameIterator, ImageFormat, ImageSequence as RilImageSequence,
};

use crate::{error::Error, image::Image, types::DisposalMethod, Xy};

/// Represents a frame in an image sequence. It encloses :class:`.Image` and extra metadata about the frame.
///
/// Parameters
/// ----------
/// image: :class:`.Image`
///     The image used for this frame.
#[derive(Clone)]
#[pyclass]
#[pyo3(text_signature = "(image)")]
pub struct Frame {
    inner: RilFrame<Dynamic>,
}

#[pymethods]
impl Frame {
    #[new]
    fn new(image: Image) -> Self {
        Self {
            inner: RilFrame::from_image(image.inner),
        }
    }

    /// int: Returns the delay duration for this frame.
    #[getter]
    fn get_delay(&self) -> u128 {
        self.inner.delay().as_millis()
    }

    /// Tuple[int, int]: Returns the dimensions of this frame.
    #[getter]
    fn get_dimensions(&self) -> Xy {
        self.inner.dimensions()
    }

    /// :class:`.DisposalMethod`: Returns the disposal method for this frame.
    #[getter]
    fn get_disposal(&self) -> DisposalMethod {
        self.inner.disposal().into()
    }

    /// :class:`.Image`: Returns the image this frame contains.
    #[getter]
    fn get_image(&self) -> Image {
        Image {
            inner: self.inner.image().clone(),
        }
    }

    #[setter]
    fn set_delay(&mut self, delay: u64) {
        self.inner.set_delay(Duration::from_millis(delay));
    }

    #[setter]
    fn set_disposal(&mut self, disposal: DisposalMethod) {
        self.inner.set_disposal(disposal.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "<Frame delay={} dimensions=({}, {}) disposal={}>",
            self.get_delay(),
            self.get_dimensions().0,
            self.get_dimensions().1,
            self.get_disposal()
        )
    }
}

/// Represents a sequence of image frames such as an animated image.
///
/// See :class:`.Image` for the static image counterpart, and see :class:`.Frame` to see how each frame is represented in an image sequence.
///
/// The iterator is exhausive, so when you iterate through :class:`.ImageSequence` like
///
/// .. code-block:: python3
///
///     seq = ImageSequence.from_bytes(bytes)
///     list(seq) # [...]
///     # But if you do it again
///     list(seq) # []
///     # It will return a empty list
///
/// .. note::
///     Any change made to the :class:`.Frame` will not be reflected to the :class:`.ImageSequence`, so you must create a new :class:`.ImageSequence` after you make changes to the frames.
#[pyclass]
pub struct ImageSequence {
    inner: RilImageSequence<Dynamic>,
    iter: Box<dyn Iterator<Item = ril::Frame<Dynamic>> + Send>,
}

#[pymethods]
impl ImageSequence {
    /// Decodes a sequence with the explicitly given image encoding from the raw bytes.
    ///
    /// if `format` is not provided then it will try to infer its encoding.
    ///
    /// Parameters
    /// ----------
    /// bytes: bytes
    ///     The bytes of the image.
    /// format: Optional[str], default: None
    ///     The format of the image.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The format provided is invalid.
    /// RuntimeError
    ///     Failed to decode the image or Failed to infer the image's format.
    #[classmethod]
    #[pyo3(text_signature = "(cls, bytes, format)")]
    fn from_bytes(_: &PyType, bytes: &[u8], format: Option<&str>) -> Result<Self, Error> {
        Ok(if let Some(format) = format {
            let inner = RilImageSequence::from_bytes(ImageFormat::from_extension(format)?, bytes)?
                .into_sequence()?;
            let iter = Box::new(inner.clone().into_iter());

            Self { inner, iter }
        } else {
            let inner = RilImageSequence::from_bytes(ImageFormat::infer_encoding(bytes), bytes)?
                .into_sequence()?;
            let iter = Box::new(inner.clone().into_iter());

            Self { inner, iter }
        })
    }

    /// Creates a new image sequence from the given frames
    ///
    /// Parameters
    /// ----------
    /// frames: List[:class:`Frame`]
    ///     The list of frames to create the sequence from
    #[classmethod]
    fn from_frames(_: &PyType, frames: Vec<Frame>) -> Self {
        let inner =
            RilImageSequence::from_frames(frames.into_iter().map(|x| x.inner).collect::<Vec<_>>());
        let iter = Box::new(inner.clone().into_iter());

        Self { inner, iter }
    }

    /// Opens a file from the given path and decodes it into an :class:`.ImageSequence`.
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
        let inner = RilImageSequence::open(path)?.into_sequence()?;
        let iter = Box::new(inner.clone().into_iter());
        Ok(Self { inner, iter })
    }

    /// Encodes the image with the given encoding and returns `bytes`.
    ///
    /// Parameters
    /// ----------
    /// encoding: str
    ///     The encoding to encode to.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     The encoded bytes.
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
    ///     The path to the image.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     The file extension is invalid.
    /// RuntimeError
    ///     Failed to infer file format or Failed to decode image.
    fn save(&self, path: PathBuf, encoding: Option<&str>) -> Result<(), Error> {
        if let Some(encoding) = encoding {
            let encoding = ImageFormat::from_extension(encoding)?;
            self.inner.save(encoding, path)?;
        } else {
            self.inner.save_inferred(path)?;
        }

        Ok(())
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Frame> {
        slf.iter.next().map(|x| Frame { inner: x })
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!("<ImageSequence len={}>", self.__len__())
    }
}
