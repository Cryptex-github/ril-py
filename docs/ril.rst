.. currentmodule:: ril

Ril API Reference
=================


Ril provide a performant and high-level image processing library for Python written in Rust.


Image
-----

.. autoclass:: Image
    :members:


Pixel
-----

There are two pixel types.

:class:`Pixel` and other pixel classes.

:class:`Pixel` is what the user creates, to represent the pixel type they desire.

Other pixel types are usually returned from the library.

This is done due to some limitation between converting types.

.. autoclass:: BitPixel
    :members:

.. autoclass:: L
    :members:

.. autoclass:: Rgb
    :members:

.. autoclass:: Rgba
    :members:

.. autoclass:: Pixel
    :members:


Draw
----

.. autoclass:: Border
    :members:

.. autoclass:: Rectangle
    :members:

.. autoclass:: Ellipse
    :members:


Sequence
--------

.. autoclass:: ImageSequence
    :members:

.. autoclass:: Frame
    :members:


Text
----

.. autoclass:: Font
    :members:

.. autoclass:: TextSegment
    :members:

.. autoclass:: TextLayout
    :members:


Enums
-----

.. class:: DisposalMethod

    The method used to dispose a frame before transitioning to the next frame in an image sequence.

    .. attribute:: Keep

        Do not dispose the current frame. Usually not desired for transparent images.
    
    .. attribute:: Background

        Dispose the current frame completely and replace it with the image's background color.
    
    .. attribute:: Previous

        Dispose and replace the current frame with the previous frame.

.. class:: ResizeAlgorithm

    A filtering algorithm that is used to resize an image.
    
    .. attribute:: Nearest

        A simple nearest neighbor algorithm. Although the fastest, this gives the lowest quality resizings.
        
        When upscaling this is good if you want a "pixelated" effect with no aliasing. 
        
    .. attribute:: Box

        A box filter algorithm. Equivalent to the :attr:`Nearest` filter if you are upscaling.
    
    .. attribute:: Bilinear

        A bilinear filter. Calculates output pixel value using linear interpolation on all pixels.
    
    .. attribute:: Hamming

        While having similar performance as the :attr:`Bilinear` filter, this produces a sharper and usually considered better quality image than the :attr:`Bilinear` filter, but only when downscaling. This may give worse results than bilinear when upscaling.
    
    .. attribute:: Bicubic

        A Catmull-Rom bicubic filter, which is the most common bicubic filtering algorithm. Just like all cubic filters, it uses cubic interpolation on all pixels to calculate output pixels.
    
    .. attribute:: Mitchell
        
        A Mitchell-Netravali bicubic filter. Just like all cubic filters, it uses cubic interpolation on all pixels to calculate output pixels.
    
    .. attribute:: Lanczos3

        A Lanczos filter with a window of 3. Calculates output pixel value using a high-quality Lanczos filter on all pixels.

.. class:: WrapStyle

    The wrapping style of text.

    .. attribute:: NoWrap

        Do not wrap text.
    
    .. attribute:: Word

        Wrap text on word boundaries.
    
    .. attribute:: Character

        Wrap text on character boundaries.

.. class:: OverlayMode

    The mode to use when overlaying an image onto another image.

    .. attribute:: Overwrite

        Overwrite the pixels of the image with the pixels of the overlay image.
    
    .. attribute:: Blend

        Blend the pixels of the image with the pixels of the overlay image.

.. class:: HorizontalAnchor
    
    The horizontal anchor of text.

    .. attribute:: Left

        Anchor text to the left.
    
    .. attribute:: Center

        Anchor text to the center.
    
    .. attribute:: Right

        Anchor text to the right.

.. class:: VerticalAnchor
        
    The vertical anchor of text.

    .. attribute:: Top

        Anchor text to the top.

    .. attribute:: Center

        Anchor text to the center.

    .. attribute:: Bottom

        Anchor text to the bottom.
