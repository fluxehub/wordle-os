#![no_std]
#![no_main]

mod draw_target;

use core::panic::PanicInfo;
use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, primitives::Rectangle, text::Text};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use profont::PROFONT_9_POINT;

use draw_target::{VGAColor, VGATarget};

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = VGATarget::init();

    // Pallete test
    // for i in 0..=0xFF_i32 {
    //     let square = Rectangle::new(Point::new(i % 16 * 10, i / 16 * 10), Size::new(10, 10))
    //         .into_styled(PrimitiveStyle::with_fill(VGAColor::from(i as u8)));

    //     square.draw(&mut display).unwrap();
    // }

    let text_style = MonoTextStyle::new(&PROFONT_9_POINT, VGAColor::from(0x0F));
    let text = Text::new("Hello World!", Point::zero(), text_style);

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
