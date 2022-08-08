use std::{path::PathBuf, time::Duration};

use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};
use ril::{
    Dynamic, Frame as RilFrame, FrameIterator, ImageFormat, ImageSequence as RilImageSequence,
};

use crate::{error::Error, image::Image, types::DisposalMethod, Xy};

#[derive(Clone)]
#[pyclass]
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

    #[getter]
    fn get_delay(&self) -> u128 {
        self.inner.delay().as_millis()
    }

    #[getter]
    fn get_dimensions(&self) -> Xy {
        self.inner.dimensions()
    }

    #[getter]
    fn get_disposal(&self) -> DisposalMethod {
        self.inner.disposal().into()
    }

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
    #[classmethod]
    fn from_bytes(_: &PyType, bytes: &[u8], format: Option<&str>) -> Result<Self, Error> {
        Ok(if let Some(format) = format {
            let inner =
                RilImageSequence::decode_from_bytes(ImageFormat::from_extension(format)?, bytes)?
                    .into_sequence()?;
            let iter = Box::new(inner.clone().into_iter());

            Self { inner, iter }
        } else {
            let inner =
                RilImageSequence::decode_from_bytes(ImageFormat::infer_encoding(bytes), bytes)?
                    .into_sequence()?;
            let iter = Box::new(inner.clone().into_iter());

            Self { inner, iter }
        })
    }

    #[classmethod]
    fn from_frames(_: &PyType, frames: Vec<Frame>) -> Self {
        let inner =
            RilImageSequence::from_frames(frames.into_iter().map(|x| x.inner).collect::<Vec<_>>());
        let iter = Box::new(inner.clone().into_iter());

        Self { inner, iter }
    }

    /// Encodes the image with the given encoding and returns `bytes`.
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
    /// You can try saving to a memory buffer by using the encode method.
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
