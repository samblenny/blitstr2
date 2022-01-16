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

pub fn small_glyph_offset(codepoint: char) -> Result<usize, usize> {
    match small::CODEPOINTS.binary_search(&(codepoint as u32)) {
        Ok(n) => Ok(n << 3),
        _ => Err(0),
    }
}

pub fn small_glyph(offset: usize) -> Result<&'static [u32], usize> {
    match offset + 8 < small::GLYPHS.len() {
        true => Ok(&small::GLYPHS[offset..offset + 8]),
        false => Err(0),
    }
}

pub fn regular_glyph_offset(codepoint: char) -> Result<usize, usize> {
    match regular::CODEPOINTS.binary_search(&(codepoint as u32)) {
        Ok(n) => Ok(n << 3),
        _ => Err(0),
    }
}

pub fn regular_glyph(offset: usize) -> Result<&'static [u32], usize> {
    match offset + 8 < regular::GLYPHS.len() {
        true => Ok(&regular::GLYPHS[offset..offset + 8]),
        false => Err(0),
    }
}

pub fn emoji_glyph_offset(codepoint: char) -> Result<usize, usize> {
    match emoji::CODEPOINTS.binary_search(&(codepoint as u32)) {
        Ok(n) => Ok(n << 3),
        _ => Err(0),
    }
}

pub fn emoji_glyph(offset: usize) -> Result<&'static [u32], usize> {
    match offset + 8 < emoji::GLYPHS.len() {
        true => Ok(&emoji::GLYPHS[offset..offset + 8]),
        false => Err(0),
    }
}
