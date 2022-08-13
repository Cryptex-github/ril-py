use ril::{TextSegment as RilTextSegment, TextLayout as RilTextLayout, Dynamic, Draw, Font as RilFont};
use pyo3::{prelude::*, exceptions::PyRuntimeError, types::PyType};

use std::{sync::Arc, marker::PhantomData, path::PathBuf};

use crate::{error::Error, Xy, utils::cast_pixel_to_pyobject};

/// Represents a text segment that can be drawn.
/// 
/// See :class:`TextLayout` for a more robust implementation that supports rendering text with multiple styles. 
/// This type is for more simple and lightweight usages.
/// 
/// Additionally, accessing metrics such as the width and height of the text cannot be done here, 
/// but can be done in TextLayout since it keeps a running copy of the layout. 
/// Use TextLayout if you will be needing to calculate the width and height of the text. 
/// Additionally, TextLayout supports text anchoring, which can be used to align text.
/// 
/// If you need none of these features, :class:`TextSegment` should be used in favor of being much more lightweight.
#[pyclass]
#[derive(Clone)]
pub struct TextSegment {
    pub(crate) inner: RilTextSegment<'static, Dynamic>
}


/// Represents a high-level text layout that can layout text segments, maybe with different fonts.
/// 
/// This is a high-level layout that can be used to layout text segments. 
/// It can be used to layout text segments with different fonts and styles, and has many features over :class:`TextSegment` such as text anchoring, 
/// which can be useful for text alignment. 
/// This also keeps track of font metrics, meaning that unlike :class:`TextSegment`, 
/// this can be used to determine the width and height of text before rendering it.
/// 
/// This is less efficient than :class:`TextSegment` and you should use :class:`TextSegment` if you don't need any of the features TextLayout provides.
#[pyclass]
#[derive(Clone)]
pub struct TextLayout {
    pub(crate) inner: Arc<RilTextLayout<'static, Dynamic>>
}


/// Represents a single font along with its alternatives used to render text. Currently, this supports TrueType and OpenType fonts.
#[pyclass]
pub struct Font {
    inner: RilFont
}

#[pyclass]
pub enum WrappingStyle {
    Nothing,
    Word,
    Character
}

#[pymethods]
impl TextSegment {
    #[getter]
    fn position(&self) -> Xy {
        self.inner.position
    }

    #[getter]
    fn width(&self) -> Option<u32> {
        self.inner.width
    }

    #[getter]
    fn text(&self) -> String {
        self.inner.text.clone()
    }

    #[getter]
    fn font(&self) -> Font {
        let font = self.inner.font as *const ril::Font;

        Font {
            inner: unsafe {
                std::ptr::read(font)
            }
        }
    }

    #[getter]
    fn fill(&self, py: Python<'_>) -> PyObject {
        cast_pixel_to_pyobject(py, self.inner.fill)
    }

    #[getter]
    fn overlay(&self) -> String {
        self.inner.overlay.to_string()
    }

    #[getter]
    fn size(&self) -> f32 {
        self.inner.size
    }

}

#[pymethods]
impl Font {
    /// Opens the font from the given path.
    /// 
    /// .. note::
    ///     The optimal size is not the fixed size of the font - rather it is the size to optimize rasterizing the font for.
    /// 
    ///     Lower sizes will look worse but perform faster, while higher sizes will look better but perform slower. 
    ///     It is best to set this to the size that will likely be the most use
    /// 
    /// Parameters
    /// ----------
    /// path: str
    ///     The path of the font.
    /// optimal_size: float
    ///     The optimal size of the font.
    /// 
    /// Raises
    /// ------
    /// IOError
    ///     Fails to read the font file.
    /// RuntimeError
    ///     Fails to load the font.
    /// 
    /// .. seealso::
    ///     :meth:`from_bytes`
    #[classmethod]
    #[pyo3(text_signature = "cls, path, optimal_size")]
    fn open(_: &PyType, path: PathBuf, optimal_size: f32) -> Result<Self, Error> {
        Ok(Self {
            inner: RilFont::open(path, optimal_size)?
        })
    }

    /// Loads the font from the given bytes.
    /// 
    /// .. note::
    ///     The optimal size is not the fixed size of the font - rather it is the size to optimize rasterizing the font for.
    /// 
    ///     Lower sizes will look worse but perform faster, while higher sizes will look better but perform slower. 
    ///     It is best to set this to the size that will likely be the most use
    /// 
    /// Parameters
    /// ----------
    /// path: str
    ///     The path of the font.
    /// optimal_size: float
    ///     The optimal size of the font.
    /// 
    /// Raises
    /// ------
    /// IOError
    ///     Fails to read the font file.
    /// RuntimeError
    ///     Fails to load the font.
    #[classmethod]
    #[pyo3(text_signature = "cls, bytes, optimal_size")]
    fn from_bytes(_: &PyType, bytes: &[u8], optimal_size: f32) -> Result<Self, Error> {
        Ok(Self {
            inner: RilFont::from_bytes(bytes, optimal_size)?
        })
    }

    /// float: Returns the optimal size, in pixels, of this font.
    /// 
    /// ..note::
    ///     The optimal size is not the fixed size of the font - rather it is the size to optimize rasterizing the font for.
    /// 
    ///     Lower sizes will look worse but perform faster, while higher sizes will look better but perform slower. 
    ///     It is best to set this to the size that will likely be the most used.
    #[getter]
    fn optimal_size(&self) -> f32 {
        self.inner.optimal_size()
    }
}


macro_rules! impl_shared_draw_entities {
    ($obj:expr, $( $class:ty ),*) => {{
        $(
            match $obj.extract::<$class>() {
                Ok(r) => return Ok(Self(r.inner, PhantomData)),
                Err(_) => ()
            }
        )*

        Err(PyRuntimeError::new_err(
            "Invalid argument for draw".to_string(),
        ))
    }};
}

pub struct SharedDrawEntity<'a>(pub Arc<dyn Draw<Dynamic>>, PhantomData<&'a ()>);

impl<'a> FromPyObject<'a> for SharedDrawEntity<'a> {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        impl_shared_draw_entities!(obj, TextLayout)
    }
}
