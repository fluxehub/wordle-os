use embedded_graphics::{
    image::ImageRaw,
    mono_font::{
        mapping::{self, StrGlyphMapping},
        DecorationDimensions, MonoFont,
    },
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

// pub const COZETTE: MonoFont = MonoFont {
//     image: ImageRaw::new_binary(DATA, 1),
//     glyph_mapping: &StrGlyphMapping::new(" ", 0),
//     character_size: Size::new(1, 1),
//     character_spacing: 4,
//     baseline: 13,
//     underline: DecorationDimensions::default_underline(13),
//     strikethrough: DecorationDimensions::default_strikethrough(13),
// };
