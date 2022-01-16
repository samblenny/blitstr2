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

pub fn small_glyph(ch: char) -> Result<&'static [u32], usize> {
    match small::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end < small::GLYPHS.len() {
                true => Ok(&small::GLYPHS[offset..end]),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn regular_glyph(ch: char) -> Result<&'static [u32], usize> {
    match regular::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end < regular::GLYPHS.len() {
                true => Ok(&regular::GLYPHS[offset..end]),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}

pub fn emoji_glyph(ch: char) -> Result<&'static [u32], usize> {
    match emoji::CODEPOINTS.binary_search(&(ch as u32)) {
        Ok(n) => {
            let offset = n << 3;
            let end = offset + 8;
            match end < emoji::GLYPHS.len() {
                true => Ok(&emoji::GLYPHS[offset..end]),
                false => Err(0),
            }
        }
        _ => Err(1),
    }
}
