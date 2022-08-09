from __future__ import annotations

from typing import List, Type


class Image:
    @classmethod
    def new(cls: Type[Image], width: int, height: int, fill: Pixel) -> Image: ...

    @classmethod
    def from_bytes(cls: Type[Image], bytes: bytes, format: str | None = None) -> Image: ...

    @classmethod
    def from_pixels(cls: Type[Image], width: int, pixels: List[Pixel]) -> Image: ...

    @classmethod
    def open(cls: Type[Image], path: str) -> Image: ...


class Pixel:
    @classmethod
    def from_bitpixel(cls: Type[Pixel], value: bool) -> Pixel: ...
