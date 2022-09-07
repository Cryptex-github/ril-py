from ril import Image, ImageSequence, Pixel, Rgba

PIXELS = [
    Rgba(255, 0, 0, 255),
    Rgba(255, 128, 0, 255),
    Rgba(255, 255, 0, 255),
    Rgba(128, 255, 0, 255),
    Rgba(0, 255, 0, 255),
    Rgba(0, 255, 128, 255),
    Rgba(0, 255, 255, 255),
    Rgba(0, 128, 255, 255),
    Rgba(0, 0, 255, 255),
    Rgba(128, 0, 255, 255),
    Rgba(255, 0, 255, 255),
    Rgba(255, 0, 128, 255),
]

def test_create_image() -> None:
    image = Image.new(1, 1, Pixel.from_rgb(255, 255, 255))
    
    assert image.height == 1
    assert image.width == 1
    assert image.dimensions == (1, 1)

def test_image_pixels() -> None:
    image = Image.new(1, 1, Pixel.from_rgb(255, 255, 255))

    image.pixels()

def test_gif_decode(fetch_file) -> None:
    for i, frame in enumerate(ImageSequence.from_bytes(fetch_file('sample_rgba.gif'))):
        assert frame.dimensions == (256, 256)
        assert frame.image.get_pixel(0, 0) == PIXELS[i]
