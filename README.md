# ril
**R**ust **I**maging **L**ibrary: A high-level Rust imaging crate's python binding.

## What's this?
This is a python binding around [ril](https://github.com/jay3332/ril) designed to provide an easy-to-use, high-level interface
around image processing in Rust. Image and animation processing has never been
this easy and fast before.

## Support
⚠ This package is a work in progress and it heavily depends on the progress of [ril](https://github.com/jay3332/ril)

By the first stable release, we plan to support the following image encodings:

| Encoding Format | Current Status     |
|-----------------|--------------------|
| PNG\* (encoder) | Work in Progress   |
| PNG\* (decoder) | Work in Progress   |
| JPEG            | Not yet supported  |
| GIF             | Not yet supported  |
| WebP            | Not yet supported  |
| BMP             | Not yet supported  |
| TIFF            | Not yet supported  |

\* PNG encoding *does* account for APNG. (APNG is not yet supported)

## Installation

### Prebuilt wheels

There will be prebuilt wheels for those platforms:

* Linux x86-64: Cpython 3.7, 3.8, 3.9, 3.10
* MacOS x86-64: Cpython 3.7, 3.8, 3.9, 3.10
* Windows x86-64: Cpython 3.7, 3.8, 3.9, 3.10
* Linux i686: Cpython 3.7, 3.8, 3.9, 3.10
* Linux aarch64: Cpython 3.7, 3.8, 3.9, 3.10
* MacOS aarch64: Cpython 3.8, 3.9, 3.10

If you want another platform to have prebuilt wheels, please open an issue.

CPython 3.11 support will be available once its ABI has been stablized. 

If your platform have prebuilt wheels, installing is as simple as

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
```rs
use ril::prelude::*;

fn main() -> ril::Result<()> {
    let image = Image::open("sample.png")?;
    image.invert();
    image.save_inferred("inverted.png")?;
    
    Ok(())
}
```

or, why not use method chaining?
```rs
Image::open("sample.png")?
    .inverted()
    .save_inferred("inverted.png")?;
```

#### Create a new black image, open the sample image, and paste it on top of the black image:
```rs
let image = Image::new(600, 600, Rgb::black());
image.paste(100, 100, Image::open("sample.png")?);
image.save_inferred("sample_on_black.png")?;
```

you can still use method chaining, but this accesses a lower level interface:
```rs
let image = Image::new(600, 600, Rgb::black())
    .with(&Paste::new(Image::open("sample.png")?).with_position(100, 100))
    .save_inferred("sample_on_black.png")?;
```
