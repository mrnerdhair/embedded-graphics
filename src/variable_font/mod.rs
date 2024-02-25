//! Variable-width bitmap fonts.
//!
//! This module contains support for drawing variable-width bitmap fonts. It is based on an existing [`MonoFont`] instance, which
//! returns glyphs with a fixed width, but draws only a portion of the returned glyph whose x-coordinates are in the range
//! specified by the included [`GlyphWidthMapping`].
//! 
//! Typically, this is used to skip drawing a few extra columns of spaces at the beginning or end of a glyph's image.
//!
//! [`MonoFont`]: ../mono_font/struct.MonoFont.html
//! [`GlyphWidthMapping`]: trait.GlyphWidthMapping.html

pub mod mapping;

use core::{convert::TryInto, fmt};

use crate::{
    geometry::{OriginDimensions, Point, Size},
    mono_font::{
        DecorationDimensions, Font, MonoFont,
    },
    variable_font::{
        mapping::{GlyphWidthMapping, RangeSize},
    },
    primitives::Rectangle,
};

/// Variable-width bitmap font.
///
/// See the [module documentation] for more information about using fonts.
///
/// [module documentation]: index.html
#[derive(Clone, Copy)]
pub struct VariableFont<'a> {
    /// The underlying monospaced font.
    pub mono_font: MonoFont<'a>,

    /// Glyph width mapping.
    pub glyph_width_mapping: &'a dyn GlyphWidthMapping,
}

impl<'a> Font<'a> for VariableFont<'a> {
    type Glyph = <MonoFont<'a> as Font<'a>>::Glyph;
    fn glyph(&'a self, c: char) -> Self::Glyph {
        let mono_glyph = self.mono_font.glyph(c);
        let mono_glyph_size = mono_glyph.size();
        let glyph_width = self.glyph_width_mapping.glyph_width(&self.mono_font, c);

        mono_glyph.sub_image(&Rectangle::new(
            Point::new(glyph_width.start.try_into().unwrap(), 0),
            Size::new(glyph_width.range_size(), mono_glyph_size.height),
        ))
    }
    fn character_height(&self) -> u32 {
        self.mono_font.character_height()
    }
    fn character_spacing(&self) -> u32 {
        self.mono_font.character_spacing()
    }
    fn baseline(&self) -> u32 {
        self.mono_font.baseline()
    }
    fn strikethrough(&self) -> DecorationDimensions {
        self.mono_font.strikethrough()
    }
    fn underline(&self) -> DecorationDimensions {
        self.mono_font.underline()
    }
    fn measure_string_width(&self, text: &str) -> u32 {
        text.chars()
            .fold(0u32, |a: u32, c: char| {
                a + self
                    .glyph_width_mapping
                    .glyph_width(&self.mono_font, c)
                    .range_size()
                    + self.mono_font.character_spacing()
            })
            .saturating_sub(self.mono_font.character_spacing())
    }
}

impl PartialEq for VariableFont<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.mono_font == other.mono_font
            && core::ptr::eq(self.glyph_width_mapping, other.glyph_width_mapping)
    }
}

impl fmt::Debug for VariableFont<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariableFont")
            .field("mono_font", &self.mono_font)
            .field("glyph_width_mapping", &"?")
            // MSRV 1.53.0: use `finish_non_exhaustive`
            .finish()
    }
}
