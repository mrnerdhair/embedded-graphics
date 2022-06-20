//! Glyph width mapping.
//!
//! A glyph width mapping defines the width of characters in a [`MonoFont`] image. This is typically used
//! to skip the first or last few columns of pixels in a glyph.
//!
//! [`MonoFont`]: ../../mono_font/struct.MonoFont.html

use crate::mono_font::MonoFont;
use core::ops::Range;

/// Mapping from characters to glyph widths.
pub trait GlyphWidthMapping {
    /// Maps a character to a glyph width.
    fn glyph_width(&self, font: &MonoFont<'_>, c: char) -> Range<u32>;
}

impl<F> GlyphWidthMapping for F
where
    F: Fn(&MonoFont<'_>, char) -> Range<u32>,
{
    fn glyph_width(&self, font: &MonoFont<'_>, c: char) -> Range<u32> {
        self(font, c)
    }
}

/// A lookup-table-based glyph width mapping. 
/// 
/// The lookup table will be indexed by the glyph index returned from the MonoFont's GlyphMapping,
/// and the resulting value will be treated as the width of the glyph. No support is provided for
/// skipping space at start of the glyph or for widths which cannot be stored in a u8, but this covers
/// the majority of basic usecases.
///
/// A default width may be specified which will be returned for characters which do not have a corresponding
/// lookup table entry; if not, the character width specified by the MonoFont will be used.
#[derive(Debug, Clone)]
pub struct LookupTableGlyphWidthMapping<'a> {
    lookup_table: &'a [u8],
    default_width: Option<u32>,
}

impl<'a> LookupTableGlyphWidthMapping<'a> {
    /// Creates a new lookup-table-based glyph width mapping.
    pub const fn new(lookup_table: &'a [u8], default_width: Option<u32>) -> Self {
        Self {
            lookup_table,
            default_width,
        }
    }
}

impl GlyphWidthMapping for LookupTableGlyphWidthMapping<'_> {
    fn glyph_width(&self, font: &MonoFont<'_>, c: char) -> Range<u32> {
        let glyph_index = font.glyph_mapping.index(c);
        if glyph_index < self.lookup_table.len() {
            0..(self.lookup_table[glyph_index] as u32)
        } else {
            0..self.default_width.unwrap_or(font.character_size.width)
        }
    }
}

pub(crate) trait RangeSize {
    fn range_size(&self) -> u32;
}

impl RangeSize for Range<u32> {
    fn range_size(&self) -> u32 {
        if self.is_empty() {
            0
        } else {
            self.end - self.start
        }
    }
}
