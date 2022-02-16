#![no_std]
#![no_main]

mod cozette;
mod vga_hires_target;
mod vga_target;

use core::panic::PanicInfo;
use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, text::Text};

use vga::colors::Color16;
use vga_hires_target::{VGA16Color, VGAHiResTarget};
// use vga_target::{VGAColor, VGATarget};

use crate::cozette::COZETTE;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = VGAHiResTarget::init();

    let text_style = MonoTextStyle::new(&COZETTE, VGA16Color::from(Color16::LightRed));
    let text = Text::new("When the imposter is sus!", Point::new(2, 11), text_style);

    text.draw(&mut display).unwrap();

    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
