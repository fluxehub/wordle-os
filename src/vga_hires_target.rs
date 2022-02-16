use core::ops::Deref;
use embedded_graphics::prelude::*;
use vga::{
    colors::Color16,
    writers::{Graphics640x480x16, GraphicsWriter},
};

pub struct VGAHiResTarget {
    pub writer: Graphics640x480x16,
}

impl VGAHiResTarget {
    pub fn init() -> Self {
        let writer = Graphics640x480x16::new();

        writer.set_mode();
        writer.clear_screen(Color16::Black);

        Self { writer }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VGA16Color(Color16);

impl PixelColor for VGA16Color {
    type Raw = ();
}

impl From<Color16> for VGA16Color {
    fn from(color: Color16) -> Self {
        Self(color)
    }
}

impl Deref for VGA16Color {
    type Target = Color16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DrawTarget for VGAHiResTarget {
    type Color = VGA16Color;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.writer
                .set_pixel(coord.x as usize, coord.y as usize, *color);
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.writer.clear_screen(*color);

        Ok(())
    }
}

impl OriginDimensions for VGAHiResTarget {
    fn size(&self) -> Size {
        Size::new(640, 480)
    }
}
