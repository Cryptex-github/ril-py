use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyType};
use ril::{Draw, Dynamic, Font as RilFont};

use std::{marker::PhantomData, path::PathBuf, sync::{Arc, RwLock}};

use crate::{
    error::Error,
    pixels::Pixel,
    workaround::{OwnedTextSegment as RilTextSegment, OwnedTextLayout as RilTextLayout},
    types::{HorizontalAnchor, OverlayMode, VerticalAnchor, WrapStyle},
    utils::cast_pixel_to_pyobject,
    Xy,
};

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
/// 
/// Parameters
/// ----------
/// font: :class:`Font`
///     The font to use to render the text.
/// text: str
///     The text to render.
/// fill: :class:`Pixel`
///     The fill color the text will be in.
/// position: Optional[Tuple[int, int]]
///     The position the text will be rendered at.
/// 
///     **This must be set before adding any text segments!**
/// 
///     Either with :attr:`position` or by passing it to the constructor.
/// size: Optional[float]
///     The size of the text in pixels.
/// overlay: Optional[:class:`OverlayMode`]
///    The overlay mode to use when rendering the text.
/// width: Optional[int]
///    The width of the text layout.
/// wrap: Optional[:class:`WrapStyle`]
///     The wrapping style of the text. Note that text will only wrap if `width` is set.
///     If this is used in a :class:`TextLayout`, this is ignored and :attr:`.WrapStyle.Wrap` is used instead.
///
/// 
/// .. warning::
///     As this class contains the data of an entire font, copying this class is expensive.
#[pyclass]
#[derive(Clone)]
pub struct TextSegment {
    pub(crate) inner: RilTextSegment<Dynamic>,
}

#[pymethods]
impl TextSegment {
    #[new]
    fn new(
        font: Font,
        text: &str,
        fill: Pixel,
        position: Option<Xy>,
        size: Option<f32>,
        overlay: Option<OverlayMode>,
        width: Option<u32>,
        wrap: Option<WrapStyle>,
    ) -> Self {
        let font_size = font.optimal_size();

        let mut inner = RilTextSegment::new(font.inner, text, fill.inner);

        inner.position = position.unwrap_or((0, 0));
        inner.size = size.unwrap_or(font_size);
        inner.overlay = overlay.unwrap_or(OverlayMode::Merge).into();
        inner.width = width;
        inner.wrap = wrap.unwrap_or(WrapStyle::Word).into();

        Self { inner }
    }

    /// Tuple[int, int]: The position of the text segment.
    #[getter]
    fn position(&self) -> Xy {
        self.inner.position
    }

    /// float: The width of the text box.
    ///
    /// .. warning::
    ///     If this is used in a :class:`TextLayout`, this is ignored and :meth:`TextLayout.width` is used instead.
    #[getter]
    fn width(&self) -> Option<u32> {
        self.inner.width
    }

    /// str: The content of the text segment.
    #[getter]
    fn text(&self) -> String {
        self.inner.text.clone()
    }

    /// :class:`Font`: The font of the text segment.
    ///
    /// .. warning::
    ///     Due to design limitation, accessing font requires a deep clone each time, which is expensive.
    #[getter]
    fn font(&self) -> Font {
        Font {
            inner: self.inner.font.clone(),
        }
    }

    /// List[List[Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]]]: The fill color of the text segment.
    #[getter]
    fn fill(&self, py: Python<'_>) -> PyObject {
        cast_pixel_to_pyobject(py, self.inner.fill)
    }

    /// :class:`OverlayMode`: The overlay mode of the text segment.
    #[getter]
    fn overlay(&self) -> OverlayMode {
        self.inner.overlay.into()
    }

    /// float: The size of the text segment in pixels.
    #[getter]
    fn size(&self) -> f32 {
        self.inner.size
    }

    /// :class:`WrapStyle`: The wrapping style of the text segment.
    #[getter]
    fn wrap(&self) -> WrapStyle {
        self.inner.wrap.into()
    }

    #[setter]
    fn set_position(&mut self, position: Xy) {
        self.inner.position = position;
    }

    #[setter]
    fn set_width(&mut self, width: Option<u32>) {
        self.inner.width = width;
    }

    #[setter]
    fn set_text(&mut self, text: &str) {
        self.inner.text = text.to_string();
    }

    #[setter]
    fn set_font(&mut self, font: Font) {
        self.inner.font = font.inner;
    }

    #[setter]
    fn set_fill(&mut self, fill: Pixel) {
        self.inner.fill = fill.inner;
    }

    #[setter]
    fn set_overlay(&mut self, overlay: OverlayMode) {
        self.inner.overlay = overlay.into();
    }

    #[setter]
    fn set_size(&mut self, size: f32) {
        self.inner.size = size;
    }

    #[setter]
    fn set_wrap(&mut self, wrap: WrapStyle) {
        self.inner.wrap = wrap.into();
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "<TextSegment fill={}, position=({}, {}), size={}, overlay={:?}, width={}, wrap={:?}>",
            self.fill(py),
            self.position().0,
            self.position().1,
            self.size(),
            self.overlay(),
            self.width().map_or("None".to_string(), |f| f.to_string()),
            self.wrap()
        )
    }
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
///
/// Parameters
/// ----------
/// position: Optional[Tuple[int, int]]
///     The position the text will be rendered at.
/// 
///     **This must be set before adding any text segments!**
/// 
///     Either with :attr:`position` or by passing it to the constructor.

/// horizontal_anchor: Optional[:class:`.HorizontalAnchor`]
///    The horizontal anchor of the text.   
/// 
/// vertical_anchor: Optional[:class:`.VerticalAnchor`]
///     The vertical anchor of the text.

/// wrap: Optional[:class:`.WrapStyle`]
///    Sets the wrapping style of the text. Make sure to also set the wrapping width using :attr:`width` for wrapping to work.
/// 
///     **This must be set before adding any text segments!**
///
/// 
/// .. warning::
///     As this class contains the data of one or more font(s), copying this class can be extremely expensive.
#[pyclass]
#[derive(Clone)]
#[pyo3(
    text_signature = "(font, text, fill, position = None, size = None, overlay = None, width = None, wrap = None)"
)]
pub struct TextLayout {
    pub(crate) inner: Arc<RwLock<RilTextLayout<Dynamic>>>,
}

#[pymethods]
impl TextLayout {
    #[new]
    fn new(
        position: Option<Xy>,
        width: Option<u32>,
        horizontal_anchor: Option<HorizontalAnchor>,
        vertical_anchor: Option<VerticalAnchor>,
        wrap: Option<WrapStyle>,
    ) -> Self {
        let mut inner = RilTextLayout::new();

        if let Some(position) = position {
            inner.set_position(position.0, position.1);
        }

        if let Some(width) = width {
            inner.set_width(width);
        }

        if let Some(horizontal_anchor) = horizontal_anchor {
            inner.x_anchor = horizontal_anchor.into();
        }

        if let Some(vertical_anchor) = vertical_anchor {
            inner.y_anchor = vertical_anchor.into();
        }

        if let Some(wrap) = wrap {
            inner.set_wrap(wrap.into());
        }

        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    /// Sets the horizontal anchor and vertial anchor of the text to be centered. 
    /// This makes the position of the text be the center as opposed to the top-left corner.
    fn centered(&mut self) -> Result<(), Error>{
        self.inner.write()?.centered();

        Ok(())
    }

    /// Tuple[int, int, int, int]: Returns the bounding box of the text. 
    /// Left and top bounds are inclusive; right and bottom bounds are exclusive.
    #[getter]
    fn bounding_box(&self) -> Result<(u32, u32, u32, u32), Error> {
        Ok(self.inner.read()?.bounding_box())
    }

    /// Tuple[int, int]: Returns the width and height of the text.
    /// 
    /// .. warning::
    ///     This is a slightly expensive operation and is not a simple getter.
    /// 
    /// .. note::
    ///     If you want both width and height, use :attr:`dimensions`.
    #[getter]
    fn dimensions(&self) -> Result<Xy, Error> {
        Ok(self.inner.read()?.dimensions())
    }

    /// int: Returns the height of the text.
    /// 
    /// .. warning::
    ///     This is a slightly expensive operation and is not a simple getter.
    /// 
    /// .. note::
    ///     If you want both width and height, use :attr:`dimensions`.
    #[getter]
    fn height(&self) -> Result<u32, Error> {
        Ok(self.inner.read()?.height())
    }

    /// int: Returns the width of the text.
    /// 
    /// .. warning::
    ///    This is a slightly expensive operation and is not a simple getter.
    /// 
    /// .. note::
    ///    If you want both width and height, use :attr:`dimensions`.
    #[getter]
    fn width(&self) -> Result<u32, Error> {
        Ok(self.inner.read()?.width())
    }

    /// Sets the position of the text layout.
    /// 
    /// **This must be set before adding any text segments!**
    #[setter]
    fn set_position(&mut self, position: Xy) -> Result<(), Error> {
        self.inner.write()?.set_position(position.0, position.1);

        Ok(())
    }

    /// Sets the horizontal anchor of the text layout.
    #[setter]
    fn set_horizontal_anchor(&mut self, anchor: HorizontalAnchor) -> Result<(), Error> {
        self.inner.write()?.x_anchor = anchor.into();

        Ok(())
    }

    /// Sets the vertical anchor of the text layout.
    #[setter]
    fn set_vertical_anchor(&mut self, anchor: VerticalAnchor) -> Result<(), Error> {
        self.inner.write()?.y_anchor = anchor.into();

        Ok(())
    }

    /// Sets the width of the text layout.
    /// This does not impact :attr:`dimensions`.
    /// 
    /// **This must be set before adding any text segments!**
    #[setter]
    fn set_width(&mut self, width: u32) -> Result<(), Error> {
        self.inner.write()?.set_width(width);

        Ok(())
    }

    /// Sets the wrapping style of the text layout.
    /// Make sure to also set the wrapping width using :attr:`width` for wrapping to work.
    /// 
    /// **This must be set before adding any text segments!**
    #[setter]
    fn set_wrap(&mut self, wrap: WrapStyle) -> Result<(), Error> {
        self.inner.write()?.set_wrap(wrap.into());

        Ok(())
    }

    /// Pushes a basic text to the text layout.
    /// Adds basic text to the text layout. This is a convenience method that creates a :class:`TextSegment` with the given font, text, and fill and adds it to the text layout.
    /// The size of the text is determined by the font’s optimal size.
    /// 
    /// Parameters
    /// ----------
    /// font: :class:`Font`
    ///     The font to use for the text.
    /// text: str
    ///     The text to add.
    /// fill: :class:`Pixel`
    ///     The color of the text.
    #[pyo3(text_signature = "(self, font, text, fill)")]
    fn push_basic_text(&mut self, font: Font, text: &str, fill: Pixel) -> Result<(), Error> {
        self.inner.write()?.push_basic_text(font.inner, text, fill.inner);

        Ok(())
    }

    /// Pushes a text segment to the text layout.
    /// 
    /// Parameters
    /// ----------
    /// segment: :class:`TextSegment`
    ///    The text segment to add.
    #[pyo3(text_signature = "(self, segment)")]
    fn push_segment(&mut self, segment: TextSegment) -> Result<(), Error> {
        self.inner.write()?.push_segment(segment.inner);

        Ok(())
    }

    fn __repr__(&self) -> Result<String, Error> {
        let inner = self.inner.read()?;
        let bound = inner.bounding_box();
        let dimensions = inner.dimensions();

        Ok(format!(
            "<TextLayout bounding_box=({}, {}, {}, {}) dimensions=({}, {}) x_anchor={:?} y_anchor={:?}>",
            bound.0,
            bound.1,
            bound.2,
            bound.3,
            dimensions.0,
            dimensions.1,
            inner.x_anchor,
            inner.y_anchor,
        ))
    }
}

/// Represents a single font along with its alternatives used to render text. Currently, this supports TrueType and OpenType fonts.
#[pyclass]
#[derive(Clone)]
pub struct Font {
    inner: RilFont,
}

#[pymethods]
impl Font {
    /// Opens the font from the given path.
    ///
    /// .. note::
    /// 
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
    /// 
    /// .. seealso::
    ///     :meth:`from_bytes`
    #[classmethod]
    #[pyo3(text_signature = "(cls, path, optimal_size)")]
    fn open(_: &PyType, path: PathBuf, optimal_size: f32) -> Result<Self, Error> {
        Ok(Self {
            inner: RilFont::open(path, optimal_size)?,
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
    #[pyo3(text_signature = "(cls, bytes, optimal_size)")]
    fn from_bytes(_: &PyType, bytes: &[u8], optimal_size: f32) -> Result<Self, Error> {
        Ok(Self {
            inner: RilFont::from_bytes(bytes, optimal_size)?,
        })
    }

    /// float: Returns the optimal size, in pixels, of this font.
    ///
    /// .. note::
    ///     The optimal size is not the fixed size of the font - rather it is the size to optimize rasterizing the font for.
    ///
    ///     Lower sizes will look worse but perform faster, while higher sizes will look better but perform slower.
    ///     It is best to set this to the size that will likely be the most used.
    #[getter]
    fn optimal_size(&self) -> f32 {
        self.inner.optimal_size()
    }

    fn __repr__(&self) -> String {
        format!(
            "<Font optimal_size={}>",
            self.inner.optimal_size()
        )
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

pub struct SharedDrawEntity<'a>(pub Arc<RwLock<dyn Draw<Dynamic>>>, PhantomData<&'a ()>);

impl<'a> FromPyObject<'a> for SharedDrawEntity<'a> {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        impl_shared_draw_entities!(obj, TextLayout)
    }
}
