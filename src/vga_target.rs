use core::ops::Deref;
use embedded_graphics::prelude::*;
use vga::writers::{Graphics320x240x256, GraphicsWriter};

pub struct VGATarget {
    pub writer: Graphics320x240x256,
}

impl VGATarget {
    pub fn init() -> Self {
        let writer = Graphics320x240x256::new();

        writer.set_mode();
        writer.clear_screen(0x00);

        Self { writer }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VGAColor(u8);

impl PixelColor for VGAColor {
    type Raw = ();
}

impl From<u8> for VGAColor {
    fn from(color: u8) -> Self {
        Self(color)
    }
}

impl Deref for VGAColor {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DrawTarget for VGATarget {
    type Color = VGAColor;

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

impl OriginDimensions for VGATarget {
    fn size(&self) -> Size {
        Size::new(320, 240)
    }
}
