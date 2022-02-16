#![no_std]
#![no_main]

mod cozette;
mod draw_target;

use core::panic::PanicInfo;
use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, primitives::Rectangle, text::Text};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};

use draw_target::{VGAColor, VGATarget};

use crate::cozette::COZETTE;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = VGATarget::init();

    let text_style = MonoTextStyle::new(&COZETTE, VGAColor::from(0x0F));
    let text = Text::new("When the imposter is sus!", Point::zero(), text_style);

    LinearLayout::vertical(Chain::new(text))
        .align_to(
            &Rectangle::new(Point::zero(), Size::new(320, 240)),
            horizontal::Center,
            vertical::Center,
        )
        .draw(&mut display)
        .unwrap();

    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
