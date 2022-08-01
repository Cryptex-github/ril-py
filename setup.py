from setuptools import setup
from setuptools_rust import Binding, RustExtension

from pathlib import Path
this_directory = Path(__file__).parent
long_description = (this_directory / "README.md").read_text()

setup(
    name="ril",
    author="Cryptex",
    license="MIT",
    version="0.1.0",
    url="https://github.com/Cryptex-github/ril-py",
    python_requires=">=3.7",
    description="Rust Imaging Library's Python binding: A high-level image processing crate for Python written in Rust",
    long_description=long_description,
    long_description_content_type="text/markdown",
    rust_extensions=[RustExtension("ril.ril", binding=Binding.PyO3)],
    packages=["ril"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
