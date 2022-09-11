# ril-py
**R**ust **I**maging **L**ibrary for Python: Python bindings for [ril](https://github.com/jay3332/ril), a performant and high-level image processing library written in Rust.

## What's this?
This is a python binding around [ril](https://github.com/jay3332/ril) designed to provide an easy-to-use, high-level interface
around image processing in Rust. Image and animation processing has never been
this easy and fast before.

## Support
⚠ This package is a work in progress and it heavily depends on the progress of [ril](https://github.com/jay3332/ril)

By the first stable release, we plan to support the following image encodings:

| Encoding Format | Current Status     |
|-----------------|--------------------|
| PNG / APNG      |     Supported      |
| JPEG            |     Supported      |
| GIF             |     Supported      |
| WebP            | Not yet supported  |
| BMP             | Not yet supported  |
| TIFF            | Not yet supported  |

## Installation

### Prebuilt wheels

There will be prebuilt wheels for these platforms:

* Linux x86-64: Cpython 3.7, 3.8, 3.9, 3.10, PyPy 3.7, 3.8, 3.9
* MacOS x86-64: Cpython 3.7, 3.8, 3.9, 3.10, PyPy 3.7, 3.8, 3.9
* Windows x86-64: Cpython 3.7, 3.8, 3.9, 3.10, PyPy 3.7, 3.8, 3.9
* Linux i686: Cpython 3.7, 3.8, 3.9, 3.10, PyPy 3.7, 3.8, 3.9
* MacOS aarch64: Cpython 3.8, 3.9, 3.10

If you want another platform to have prebuilt wheels, please open an issue.

CPython 3.11 support will be available once its ABI has been stabilized. 

If your platform has prebuilt wheels, installing is as simple as

```
pip install ril
```

### Building from Source
In order to build from source, you will need to have the Rust compiler available in your PATH. See documentation on [https://rust-lang.org](https://rust-lang.org) to learn how to install Rust on your platform.

Then building is as simple as

```
pip install ril
```

or from Github

```
pip install git+https://github.com/Cryptex-github/ril-py
```

Pip will handle the building process.


## Examples

#### Open an image, invert it, and then save it:
```py
from ril import Image

image = Image.open("example.png")
image.invert()

image.save("example.png")
```

#### Create a new black image, open the sample image, and paste it on top of the black image:
```py
from ril import Image, Pixel

image = Image.new(600, 600, Pixel.from_rgb(0, 0, 0))
image.paste(100, 100, Image.open("sample.png"))

image.save("sample_on_black.png", "PNG") # You can also specify format if you like
```
