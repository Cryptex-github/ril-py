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

There are two pixel type.

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


Enums
-----

.. autoclass:: DisposalMethod
    :members:

.. autoclass:: ResizeAlgorithm
    :members:
