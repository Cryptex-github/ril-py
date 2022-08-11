from __future__ import annotations

from typing import Iterator, List, Optional, Tuple, Type, TypeAlias, Union

Pixels: TypeAlias = Union[BitPixel, L, Rgb, Rgba]
Xy: TypeAlias = Tuple[int, int]


class Image:
    """
    A high-level image representation.
    
    This represents a static, single-frame image. See :class:`.ImageSequence` for information on opening animated or multi-frame images.
    """
    @classmethod
    def new(cls: Type[Image], width: int, height: int, fill: Pixel) -> Image:
        """
        Creates a new image with the given width and height, with all pixels being set intially to `fill`.
        
        Parameters
        ----------
        width: int
            The width of the Image.
        height: int
            The height of the Image.
        fill: :class:`.Pixel`
            The pixel used to fill the image.
        
        Examples
        --------
        
        .. code-block:: python3
        
            Image.new(100, 100, Pixel.from_rgb(255, 255, 255))
        """

    @classmethod
    def from_bytes(cls: Type[Image], bytes: bytes, format: str | None = None) -> Image:
        """
        Decodes an image with the explicitly given image encoding from the raw bytes.
       
        if `format` is not provided then it will try to infer its encoding.
        
        Parameters
        ----------
        bytes: bytes
            The bytes of the Image.
        format: Optional[str], default: None
            The format of the image, defaults to `None`.
        
        Raises
        ------
        ValueError
            Raised if the format provided is invalid.
        RuntimeError
            Raised if the image can't be decoded or the format is unknown.
        """

    @classmethod
    def from_pixels(cls: Type[Image], width: int, pixels: List[Pixel]) -> Image:
        """
        Creates a new image shaped with the given width
        and a 1-dimensional sequence of pixels which will be shaped according to the width.
        
        Parameters
        ----------
        width: int
            The width of the image.
        pixels: List[:class:`.Pixel`]
            A List of pixels.
        """

    @classmethod
    def open(cls: Type[Image], path: str) -> Image:
        """
        Opens a file from the given path and decodes it into an image.
       
        The encoding of the image is automatically inferred.
        You can explicitly pass in an encoding by using the :meth:`from_bytes` method.
        
        Parameters
        ----------
        path: str
            The path to the image.
        
        Raises
        ------
        ValueError
            The file extension is invalid.
        RuntimeError
            Failed to infer file format or Failed to decode image.
        """

    @property
    def overlay_mode(self) -> str:
        """str: Returns the overlay mode of the image."""

    @property
    def mode(self) -> str:
        """str: Returns the mode of the image."""

    @property
    def width(self) -> int:
        """int: Returns the width of the image."""

    @property
    def height(self) -> int:
        """int: Returns the height of the image."""

    def bands(self) -> Pixels:
        """
        Return the bands of the image.
        
        Returns
        -------
        Tuple[:class:`.L`, ...]
        
        Raises
        ------
        TypeError
            The image is not of mode `RGB` or `RGBA`.
        """

    @classmethod
    def from_bands(cls: Type[Image], *bands: Union[Tuple[Rgb, ...], Tuple[Rgba, ...]]) -> Image:
        """
        Creates a new image from the given bands.
        
        Parameters
        ----------
        bands: \\* :class:`.L`
            The bands of the image.
        """

    def crop(self, x1: int, y1: int, x2: int, y2: int) -> None:
        """
        Crops this image in place to the given bounding box.
        
        Parameters
        ----------
        x1: int
            x1
        y1: int
            y1
        x2: int
            x2
        y2: int
            y2
        """

    def draw(self, entity: Union[Rectangle, Ellipse]) -> None:
        """
        Draws an object or shape onto this image.
        
        Parameters
        ----------
        entity: Union[:class:`.Rectangle`, :class:`.Ellipse`]
            The entity to draw on the image.
        """

    def resize(self, width: int, height: int, algo: ResizeAlgorithm) -> None:
        """
        Resizes this image in place to the given dimensions using the given resizing algorithm in place.
        
        Parameters
        ----------
        width: int
            The target width to resize to
        height: int
            The target height to resize to
        algo: :class:`.ResizeAlgorithm`
            The resize algorithm to use
        """

    def encode(self, encoding: str) -> bytes:
        """
        Encodes the image with the given encoding and returns `bytes`.
        
        Parameters
        ----------
        encoding: str
            The encoding of the image.
        
        Returns
        -------
        bytes
            The encoded bytes of the image.
        
        Raises
        ------
        ValueError
            The encoding is invalid.
        RuntimeError
            Failed to encode the image.
        """

    def save(self, path: str, encoding: Optional[str] = None) -> None:
        """
        Saves the image to the given path.
        If encoding is not provided, it will attempt to infer it by the path/filename's extension
        You can try saving to a memory buffer by using the :meth:`encode` method.
        
        Parameters
        ----------
        path: str
            The path to save the image to.
        encoding: Optional[str], default: None
            The encoding of the image, defaults to `None`.
        
        Raises
        ------
        ValueError
            The encoding provided is invalid.
        RuntimeError
            Failed to encode the image or Failed to infer the image format.
        """

    def pixels(self) -> List[List[Pixels]]:
        """
        Returns a 2D list representing the pixels of the image. Each list in the list is a row.
       
        For example:
       
        [[Pixel, Pixel, Pixel], [Pixel, Pixel, Pixel]]
       
        where the width of the inner list is determined by the width of the image.
       
        .. warning:: **This function involves heavy operation**
       
            This function requires multiple iterations, so it is a heavy operation for larger image.
        
        Returns
        -------
        List[List[Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]]]
            The pixels of the image.
        """

    def paste(self, x: int, y: int, image: Image, mask: Optional[Image]) -> None:
        """
        Pastes the given image onto this image at the given x and y axiss.
        
        If `maske` is provided it will be masked with the given masking image.
        
        Currently, only BitPixel images are supported for the masking image.
        
        Parameters
        ----------
        x: int
            The x axis
        y: int
            The y axis
        image: :class:`Image`
            The image to paste.
        mask: Optional[:class:`Image`], default: None
            The mask to use, defaults to `None`
        
        Raises
        ------
        ValueError
            The mask provided is not of mode `BitPixel`
        """

    def mask_alpha(self, mask: Image) -> None:
        """
        Masks the alpha values of this image with the luminance values of the given single-channel L image.
       
        If you want to mask using the alpha values of the image instead of providing an L image, you can split the bands of the image and extract the alpha band.
       
        This masking image must have the same dimensions as this image.
        
        Parameters
        ----------
        mask: :class:`Image`
            The mask to use
        
        Raises
        ------
        ValueError
            The mask provided is not of mode `L`
        """

    def mirror(self) -> None:
        """Mirrors, or flips this image horizontally (about the y-axis) in place."""

    def flip(self) -> None:
        """Flips this image vertically (about the x-axis) in place."""

    @property
    def format(self) -> str:
        """
        str: Returns the encoding format of the image.
        
        .. note::
            This is nothing more but metadata about the image.
            When saving the image, you will still have to explicitly specify the encoding format.
        """

    @property
    def dimensions(self) -> Tuple[int, int]:
        """Tuple[int, int]: Returns the dimensions of the image."""

    def get_pixel(self, x: int, y: int) -> Pixels:
        """
        Returns the pixel at the given coordinates.
        
        Parameters
        ----------
        x: int
            The x axis
        y: int
            The y axis
        
        Returns
        -------
        Union[:class:`.BitPixel`, :class:`.L`, :class:`.Rgb`, :class:`.Rgba`]
            The pixel of that specific coordinate.
        """

    def set_pixel(self, x: int, y: int, pixel: Pixel) -> None:
        """
        Sets the pixel at the given coordinates to the given pixel.
        
        Parameters
        ---------
        x: int
            The x axis
        y: int
            The y axis
        pixel: :class:`.Pixel`
            The pixel to set it to
        """

    def invert(self) -> None:
        """Inverts the image in-place."""


class Border:
    """
    Represents a shape border.
    """
    color: Pixel
    thickness: int
    position: str

    def __init__(self, color: Pixel, thickness: int, position: str) -> None:
        """
    Parameters
    ----------
    color: :class:`.Pixel`
        The color of the border
    thickness: int
        The thickness of the border
    position: str
        The position of the border
    
    Raises
    ------
    ValueError
        The position is not one of `inset`, `center`, or `outset`
        """


class Ellipse:
    """
    An ellipse, which could be a circle.
    
    .. warning::
        Using any of the predefined constructors will automatically set the position to (0, 0) and you must explicitly set the size of the ellipse with `.size` in order to set a size for the ellipse. 
        A size must be set before drawing.
    
        This also does not set any border or fill for the ellipse, you must explicitly set either one of them.
    """
    position: Xy
    radii: Xy
    border: Optional[Border]
    fill: Optional[Pixel]
    overlay: Optional[str]

    def __init__(
        self,
        position: Xy,
        radii: Xy,
        border: Optional[Border] = None,
        fill: Optional[Pixel] = None,
        overlay: Optional[str] = None
    ) -> None:
        """
        Parameters
        ---------
        position: Tuple[int, int]
            The position of the ellipse
        radii: Tuple[int, int]
            The radii of the ellipse
        border: Optional[:class:`.Border`]
            The border of the ellipse.
        fill: Optional[:class:`.Pixel`]
            The color to use for filling the ellipse
        overlay: Optional[str]
            The overlay mode of the ellipse.
        
        Raises
        ------
        ValueError
            The overlay mode provided is not one of `replace`, or `merge`
        """

    @classmethod
    def from_bounding_box(cls, x1: int, y1: int, x2: int, y2: int) -> Ellipse:
        """
        Creates a new ellipse from the given bounding box.
        
        Parameters
        ----------
        x1: int
        y1: int
        x2: int
        y2: int
        """

    @classmethod
    def circle(cls, x: int, y: int, radius: int) -> Ellipse:
        """
        Creates a new circle with the given center position and radius.
        
        Parameters
        ----------
        x: int
            The x axis
        y: int
            The y axis
        radius: int
            The radius
        """


class Rectangle:
    """
    A rectangle.
    
    .. warning::
        Using any of the predefined construction methods will automatically set the position to (0, 0). 
        If you want to specify a different position, you must set the position with `.position`
    
        You must specify a width and height for the rectangle with something such as with_size. 
        If you don't, a panic will be raised during drawing. 
        You can also try using from_bounding_box to create a rectangle from a bounding box, which automatically fills in the size.
    
        Additionally, a panic will be raised during drawing if you do not specify either a fill color or a border.
        these can be set with `.fill` and `.border` respectively.
    """
    position: Xy
    size: Xy
    border: Optional[Border]
    fill: Optional[Pixel]
    overlay: Optional[str]

    def __init__(
        self,
        position: Xy,
        size: Xy,
        border: Optional[Border] = None,
        fill: Optional[Pixel] = None,
        overlay: Optional[str] = None
    ) -> None: ...

    @classmethod
    def from_bounding_box(cls, x1: int, y1: int, x2: int, y2: int) -> Rectangle:
        """
    Parameters
    ----------
    position: Tuple[int, int]
        The position of the rectangle
    size: Tuple[int, int]
        The size of the rectangle
    border: Optional[:class:`.Border`]
        The border of the ellipse.
    fill: Optional[:class:`.Pixel`]
        The color to use for filling the rectangle
    overlay: Optional[str]
        The overlay mode of the rectangle.
    
    Raises
    ------
    ValueError
        The overlay mode provided is not one of `replace`, or `merge`
        """


class BitPixel:
    """Represents a single-bit pixel that represents either a pixel that is on or off."""
    value: bool

    def __init__(self, value: bool) -> None: ...


class L:
    """
    Represents an L, or luminance pixel that is stored as only one single number representing how bright, or intense, the pixel is. 
    
    This can be thought of as the “unit channel” as this represents only a single channel in which other pixel types can be composed of.
    """
    value: int

    def __init__(self, value: int) -> None: ...


class Rgb:
    """Represents an RGB pixel."""
    r: int
    g: int
    b: int

    def __init__(self, r: int, g: int, b: int) -> None: ...


class Rgba:
    """Represents an RGBA pixel."""
    r: int
    g: int
    b: int
    a: int

    def __init__(self, r: int, g: int, b: int, a: int) -> None: ...



class Pixel:
    """The user created Pixel type."""
    @classmethod
    def from_bitpixel(cls, value: bool) -> Pixel:
        """
        Create a bitpixel.
        
        Parameters
        ----------
        value: bool
            Whether the pixel is on.
        """

    @classmethod
    def from_l(cls, value: int) -> Pixel:
        """
        Create a L Pixel.
        
        Parameters
        ----------
        value: int
            The luminance value of the pixel, between 0 and 255.
        """

    @classmethod
    def from_rgb(cls, r: int, g: int, b: int) -> Pixel:
        """
        Creates a Rgb Pixel
        
        Parameters
        ----------
        r: int
            The red component of the pixel.
        g: int
            The green component of the pixel.
        b: int
            The blue component of the pixel.
        """

    @classmethod
    def from_rgba(cls, r: int, g: int, b: int, a: int) -> Pixel:
        """
        Creates a Rgba Pixel
        
        Parameters
        ----------
        r: int
            The red component of the pixel.
        g: int
            The green component of the pixel.
        b: int
            The blue component of the pixel.
        a: int
            The alpha component of the pixel.
        """


class Frame:
    """Represents a frame in an image sequence. It encloses :class:`.Image` and extra metadata about the frame."""
    def __init__(self, image: Image) -> None:
        """
    Parameters
    ----------
    image: :class:`.Image`
        The image used for this frame.
        """

    @property
    def delay(self) -> int:
        """int: Returns the delay duration for this frame."""

    @property
    def dimensions(self) -> Xy:
        """Tuple[int, int]: Returns the dimensions of this frame."""

    @property
    def disposal(self) -> DisposalMethod:
        """:class:`.DisposalMethod`: Returns the disposal method for this frame."""

    @property
    def image(self) -> Image:
        """:class:`.Image`: Returns the image this frame contains."""

    @delay.setter
    def set_delay(self, delay: int) -> None: ...


class ImageSequence(Iterator[Frame]):
    """
    Represents a sequence of image frames such as an animated image.
    
    See :class:`.Image` for the static image counterpart, and see :class:`.Frame` to see how each frame is represented in an image sequence.
    
    The iterator is exhausive, so when you iterate through :class:`.ImageSequence` like
    
    .. code-block:: python3
    
        seq = ImageSequence.from_bytes(bytes)
        list(seq) # [...]
        # But if you do it again
        list(seq) # []
        # It will return a empty list
    
    .. note::
        Any change made to the :class:`.Frame` will not be reflected to the :class:`.ImageSequence`, so you must create a new :class:`.ImageSequence` after you make changes to the frames.
    """
    @classmethod
    def from_bytes(cls, bytes: bytes, format: Optional[str] = None) -> ImageSequence:
        """
        Decodes a sequence with the explicitly given image encoding from the raw bytes.
       
        if `format` is not provided then it will try to infer its encoding.
        
        Parameters
        ----------
        bytes: bytes
            The bytes of the image.
        format: Optional[str], default: None
            The format of the image.
        
        Raises
        ------
        ValueError
            The format provided is invalid.
        RuntimeError
            Failed to decode the image or Failed to infer the image's format.
        """

    @classmethod
    def from_frames(cls, frames: List[Frame]) -> ImageSequence:
        """
        Creates a new image sequence from the given frames
        
        Parameters
        ----------
        frames: List[:class:`Frame`]
            The list of frames to create the sequence from
        """

    @classmethod
    def open(cls, path: str) -> ImageSequence:
        """
        Opens a file from the given path and decodes it into an :class:`.ImageSequence`.
       
        The encoding of the image is automatically inferred.
        You can explicitly pass in an encoding by using the :meth:`from_bytes` method.
        
        Parameters
        ----------
        path: str
            The path to the image.
        
        Raises
        ------
        ValueError
            The file extension is invalid.
        RuntimeError
            Failed to infer file format or Failed to decode image.
        """

    def encode(self, encoding: str) -> bytes:
        """
        Encodes the image with the given encoding and returns `bytes`.
        
        Parameters
        ----------
        encoding: str
            The encoding to encode to.
        
        Returns
        -------
        bytes
            The encoded bytes.
        """

    def save(self, path: str, encoding: Optional[str] = None) -> None:
        """
        Saves the image to the given path.
        If encoding is not provided, it will attempt to infer it by the path/filename's extension
        You can try saving to a memory buffer by using the :meth:`encode` method.
        
        Parameters
        ----------
        path: str
            The path to the image.
        
        Raises
        ------
        ValueError
            The file extension is invalid.
        RuntimeError
            Failed to infer file format or Failed to decode image.
        """

    def __iter__(self) -> ImageSequence: ...

    def __next__(self) -> Frame: ...


R: TypeAlias = ResizeAlgorithm


class ResizeAlgorithm:
    """A filtering algorithm that is used to resize an image."""
    Nearest: R
    Box: R
    Bilinear: R
    Hamming: R
    Bicubic: R
    Mitchell: R
    Lanczos3: R


D: TypeAlias = DisposalMethod


class DisposalMethod:
    Keep: D
    Background: D
    Previous: D
