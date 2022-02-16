use embedded_graphics::{
    image::ImageRaw,
    mono_font::{mapping, DecorationDimensions, MonoFont},
    prelude::Size,
};

pub const COZETTE: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../assets/cozette.bin"), 112),
    glyph_mapping: &mapping::ASCII,
    character_size: Size::new(7, 13),
    character_spacing: 0,
    baseline: 6,
    underline: DecorationDimensions::default_underline(13),
    strikethrough: DecorationDimensions::default_strikethrough(13),
};
