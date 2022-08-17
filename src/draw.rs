use std::{fmt::Display, marker::PhantomData};

use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
    types::PyType,
};
use ril::{
    draw::{
        Border as RilBorder, BorderPosition as RilBorderPosition, Ellipse as RilEllipse,
        Rectangle as RilRectangle,
    },
    Draw, Dynamic,
};

use crate::{
    pixels::Pixel,
    utils::{cast_pixel_to_pyobject},
    Xy, text::TextSegment, types::OverlayMode,
};

fn get_border_position(position: &str) -> PyResult<RilBorderPosition> {
    match position {
        "inset" => Ok(RilBorderPosition::Inset),
        "center" => Ok(RilBorderPosition::Center),
        "outset" => Ok(RilBorderPosition::Outset),
        _ => Err(PyValueError::new_err(
            "position provided is not valid, it must be one of `inset`, `center`, or `outset`"
                .to_string(),
        )),
    }
}

fn from_border_position(position: RilBorderPosition) -> String {
    match position {
        RilBorderPosition::Inset => "inset".to_string(),
        RilBorderPosition::Center => "center".to_string(),
        RilBorderPosition::Outset => "outset".to_string(),
    }
}

/// Represents a shape border.
///
/// Parameters
/// ----------
/// color: :class:`.Pixel`
///     The color of the border
/// thickness: int
///     The thickness of the border
/// position: str
///     The position of the border
///
/// Raises
/// ------
/// ValueError
///     The position is not one of `inset`, `center`, or `outset`
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(color, thickness, position)")]
pub struct Border {
    pub inner: RilBorder<Dynamic>,
}

#[pymethods]
impl Border {
    #[new]
    #[args("*", color, thickness, position)]
    fn new(color: Pixel, thickness: u32, position: &str) -> PyResult<Self> {
        let position = get_border_position(position)?;

        Ok(Self {
            inner: RilBorder {
                color: color.inner,
                thickness,
                position,
            },
        })
    }

    /// :class:`.Pixel`: The color of the border.
    #[getter]
    fn get_color(&self) -> Pixel {
        self.inner.color.into()
    }

    /// int: The thickness of the border, in pixels.
    #[getter]
    fn get_thickness(&self) -> u32 {
        self.inner.thickness
    }

    /// str: The position of the border.
    #[getter]
    fn get_position(&self) -> String {
        from_border_position(self.inner.position)
    }

    #[setter]
    fn set_color(&mut self, pixel: Pixel) {
        self.inner.color = pixel.inner;
    }

    #[setter]
    fn set_thickness(&mut self, thickness: u32) {
        self.inner.thickness = thickness;
    }

    #[setter]
    fn set_position(&mut self, position: &str) -> PyResult<()> {
        self.inner.position = get_border_position(position)?;

        Ok(())
    }

    fn __repr__(&self) -> String {
        format!(
            "<Border color={} thickness={} position={}>",
            self.get_color(),
            self.get_thickness(),
            self.get_position()
        )
    }
}

impl Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.__repr__())
    }
}

/// An ellipse, which could be a circle.
///
/// .. warning::
///     Using any of the predefined constructors will automatically set the position to (0, 0) and you must explicitly set the size of the ellipse with `.size` in order to set a size for the ellipse.
///     A size must be set before drawing.
///
///     This also does not set any border or fill for the ellipse, you must explicitly set either one of them.
///
/// Parameters
/// ---------
/// position: Tuple[int, int]
///     The position of the ellipse
/// radii: Tuple[int, int]
///     The radii of the ellipse
/// border: Optional[:class:`.Border`]
///     The border of the ellipse.
/// fill: Optional[:class:`.Pixel`]
///     The color to use for filling the ellipse
/// overlay: Optional[str]
///     The overlay mode of the ellipse.
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(*, position, radii, border, fill, overlay)")]
pub struct Ellipse {
    pub inner: RilEllipse<Dynamic>,
}

#[pymethods]
impl Ellipse {
    #[new]
    #[args("*", position, radii, border, fill, overlay)]
    fn new(
        position: Xy,
        radii: Xy,
        border: Option<Border>,
        fill: Option<Pixel>,
        overlay: Option<OverlayMode>,
    ) -> PyResult<Self> {
        let mut inner = RilEllipse::<Dynamic> {
            position,
            radii,
            border: None,
            fill: None,
            overlay: None,
        };

        inner.border = border.map(|i| i.inner);

        inner.fill = fill.map(|i| i.inner);

        inner.overlay = overlay.map(|i| i.into());

        Ok(Self { inner })
    }

    /// Creates a new ellipse from the given bounding box.
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
    /// 
    /// Returns
    /// -------
    /// :class:`.Ellipse`
    #[classmethod]
    #[pyo3(text_signature = "(cls, x1, y1, x2, y2)")]
    fn from_bounding_box(_: &PyType, x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self {
            inner: RilEllipse::from_bounding_box(x1, y1, x2, y2),
        }
    }

    /// Creates a new circle with the given center position and radius.
    ///
    /// Parameters
    /// ----------
    /// x: int
    ///     The x axis
    /// y: int
    ///     The y axis
    /// radius: int
    ///     The radius
    #[classmethod]
    #[pyo3(text_signature = "(cls, x, y, radius)")]
    fn circle(_: &PyType, x: u32, y: u32, radius: u32) -> Self {
        Self {
            inner: RilEllipse::circle(x, y, radius),
        }
    }

    /// Tuple[int, int]: The center position of the ellipse. The center of this ellipse will be rendered at this position.
    #[getter]
    fn get_position(&self) -> Xy {
        self.inner.position
    }

    /// Tuple[int, int]: The radii of the ellipse, in pixels; (horizontal, vertical).
    #[getter]
    fn get_radii(&self) -> Xy {
        self.inner.radii
    }

    /// Optional[:class:`.Border`]: The border of the ellipse.
    #[getter]
    fn get_border(&self) -> Option<Border> {
        self.inner
            .border
            .as_ref()
            .map(|b| Border { inner: b.clone() })
    }

    /// Optional[Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]]: The color used to fill the ellipse.
    #[getter]
    fn get_fill(&self, py: Python<'_>) -> Option<PyObject> {
        self.inner
            .fill
            .map_or(None, |fill| Some(cast_pixel_to_pyobject(py, fill)))
    }

    /// Optional[:class:`.OverlayMode`]: The overlay mode of the ellipse.
    #[getter]
    fn get_overlay(&self) -> Option<OverlayMode> {
        self.inner.overlay.map(|i| i.into())
    }

    #[setter]
    fn set_position(&mut self, position: Xy) {
        self.inner.position = position;
    }

    #[setter]
    fn set_radii(&mut self, radii: Xy) {
        self.inner.radii = radii;
    }

    #[setter]
    fn set_border(&mut self, border: Border) {
        self.inner.border = Some(border.inner);
    }

    #[setter]
    fn set_fill(&mut self, fill: Pixel) {
        self.inner.fill = Some(fill.inner);
    }

    #[setter]
    fn set_overlay(&mut self, overlay: OverlayMode) -> PyResult<()> {
        self.inner.overlay = Some(overlay.into());

        Ok(())
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "<Ellipse position=({}, {}) radii=({}, {}) border={} fill={} overlay={}>",
            self.get_position().0,
            self.get_position().1,
            self.get_radii().0,
            self.get_radii().1,
            self.get_border()
                .map_or("None".to_string(), |f| f.to_string()),
            self.get_fill(py)
                .map_or("None".to_string(), |f| f.to_string()),
            self.get_overlay()
                .map_or("None".to_string(), |f| format!("{:?}", f)),
        )
    }
}

/// A rectangle.
///
/// .. warning::
///     Using any of the predefined construction methods will automatically set the position to (0, 0).
///     If you want to specify a different position, you must set the position with `.position`
///
///     You must specify a width and height for the rectangle with something such as with_size.
///     If you don't, a panic will be raised during drawing.
///     You can also try using from_bounding_box to create a rectangle from a bounding box, which automatically fills in the size.
///
///     Additionally, a panic will be raised during drawing if you do not specify either a fill color or a border.
///     these can be set with `.fill` and `.border` respectively.
///
/// Parameters
/// ----------
/// position: Tuple[int, int]
///     The position of the rectangle
/// size: Tuple[int, int]
///     The size of the rectangle
/// border: Optional[:class:`.Border`]
///     The border of the ellipse.
/// fill: Optional[:class:`.Pixel`]
///     The color to use for filling the rectangle
/// overlay: Optional[:class:`.OverlayMode`]
///     The overlay mode of the rectangle.
///
/// Raises
/// ------
/// ValueError
///     The overlay mode provided is not one of `replace`, or `merge`
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(*, position, size, border, fill, overlay)")]
pub struct Rectangle {
    pub inner: RilRectangle<Dynamic>,
}

#[pymethods]
impl Rectangle {
    #[new]
    #[args("*", position, size, border, fill, overlay)]
    fn new(
        position: Xy,
        size: Xy,
        border: Option<Border>,
        fill: Option<Pixel>,
        overlay: Option<OverlayMode>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: RilRectangle {
                position,
                size,
                border: border.map(|b| b.inner),
                fill: fill.map(|f| f.inner),
                overlay: overlay.map(|o| o.into()),
            },
        })
    }

    /// Creates a new rectangle from two coordinates specified as 4 parameters.
    /// The first coordinate is the top-left corner of the rectangle, and the second coordinate is the bottom-right corner of the rectangle.
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
    #[classmethod]
    #[pyo3(text_signature = "(cls, x1, y1, x2, y2)")]
    fn from_bounding_box(_: &PyType, x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self {
            inner: RilRectangle::from_bounding_box(x1, y1, x2, y2),
        }
    }

    /// Tuple[int, int]: The position of the rectangle. The top-left corner of the rectangle will be rendered at this position.
    #[getter]
    fn get_position(&self) -> Xy {
        self.inner.position
    }

    /// Tuple[int, int]: The dimensions of the rectangle, in pixels.
    #[getter]
    fn get_size(&self) -> Xy {
        self.inner.size
    }

    /// :class:`.Border`: The border of the rectangle, or None if there is no border.
    #[getter]
    fn get_border(&self) -> Option<Border> {
        self.inner
            .border
            .as_ref()
            .map(|b| Border { inner: b.clone() })
    }

    /// Optional[Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]]: The color used to fill the rectangle.
    #[getter]
    fn get_fill(&self, py: Python<'_>) -> Option<PyObject> {
        self.inner
            .fill
            .map_or(None, |fill| Some(cast_pixel_to_pyobject(py, fill)))
    }

    /// Optional[:class:`.OverlayMode`]: The overlay mode of the rectangle.
    #[getter]
    fn get_overlay(&self) -> Option<OverlayMode> {
        self.inner.overlay.map(|i| i.into())
    }

    #[setter]
    fn set_position(&mut self, position: Xy) {
        self.inner.position = position;
    }

    #[setter]
    fn set_size(&mut self, size: Xy) {
        self.inner.size = size;
    }

    #[setter]
    fn set_border(&mut self, border: Option<Border>) {
        self.inner.border = border.map(|b| b.inner);
    }

    #[setter]
    fn set_fill(&mut self, fill: Option<Pixel>) {
        self.inner.fill = fill.map(|f| f.inner);
    }

    #[setter]
    fn set_overlay(&mut self, overlay: OverlayMode) -> PyResult<()> {
        self.inner.overlay = Some(overlay.into());

        Ok(())
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "<Rectangle position=({}, {}) size=({}, {}) border={} fill={} overlay={}>",
            self.get_position().0,
            self.get_position().1,
            self.get_size().0,
            self.get_size().1,
            self.get_border()
                .map_or("None".to_string(), |f| f.to_string()),
            self.get_fill(py)
                .map_or("None".to_string(), |f| f.to_string()),
            self.get_overlay()
                .map_or("None".to_string(), |f| format!("{:?}", f)),
        )
    }
}

macro_rules! impl_draw_entities {
    ($obj:expr, $( $class:ty ),*) => {{
        $(
            match $obj.extract::<$class>() {
                Ok(r) => return Ok(Self(Box::new(r.inner), PhantomData)),
                Err(_) => ()
            }
        )*

        Err(PyRuntimeError::new_err(
            "Invalid argument for draw".to_string(),
        ))
    }};
}

pub struct DrawEntity<'a>(pub Box<dyn Draw<Dynamic>>, PhantomData<&'a ()>);

impl<'a> FromPyObject<'a> for DrawEntity<'a> {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        impl_draw_entities!(obj, Rectangle, Ellipse, TextSegment)
    }
}
