// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![allow(dead_code)]
pub mod emoji;
pub mod regular;
pub mod small;

// Font data is stored as CODEPOINTS and GLYPHS arrays. CODEPOINTS holds sorted
// Unicode codepoints for characters included in the font, and GLYPHS holds
// 16*16px sprites (pixels packed in row-major order, LSB of first word is top
// left pixel of sprite). The order of codepoints and glyphs is the same, but,
// each codepoint is one u32 word long while each glyph is eight u32 words
// long. So, to find a glyph we do:
//  1. Binary search CODEPOINTS for the codepoint of interest
//  2. Multiply the codepoint index by 8, yielding an offset into GLYPHS
//  3. Slice 8 u32 words from GLYPHS starting at the offset

/// Struct to hold sprite pixel reference and associated metadata for glyphs
#[derive(Copy, Clone, Debug)]
pub struct GlyphSprite {
    pub glyph: &'static [u32],
    pub wide: u8,
    pub high: u8,
}

pub fn small_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match small::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= small::GLYPHS.len() {
                true => Ok(GlyphSprite {
                    glyph: &small::GLYPHS[offset..end],
                    wide: small::WIDTHS[n],
                    high: small::MAX_HEIGHT,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn regular_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match regular::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= regular::GLYPHS.len() {
                true => Ok(GlyphSprite {
                    glyph: &regular::GLYPHS[offset..end],
                    wide: regular::WIDTHS[n],
                    high: regular::MAX_HEIGHT,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn emoji_glyph(ch: char) -> Result<GlyphSprite, usize> {
    match emoji::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end <= emoji::GLYPHS.len() {
                true => Ok(GlyphSprite {
                    glyph: &emoji::GLYPHS[offset..end],
                    wide: emoji::MAX_HEIGHT, // yes, use height for wide
                    high: emoji::MAX_HEIGHT,
                }),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}
