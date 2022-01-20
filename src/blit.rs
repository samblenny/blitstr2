// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use crate::cliprect::ClipRect;
use crate::cursor::Cursor;
use crate::fonts;
use crate::framebuffer::{FrBuf, LINES, WIDTH, WORDS_PER_LINE};
use crate::pt::Pt;

/// Null glyph to use when everything else fails
pub const NULL_GLYPH: [u32; 8] = [0, 0x5500AA, 0x5500AA, 0x5500AA, 0x5500AA, 0x5500AA, 0, 0];
pub const NULL_GLYPH_SPRITE: fonts::GlyphSprite = fonts::GlyphSprite {
    glyph: &NULL_GLYPH,
    wide: 8u8,
    high: 12u8,
};

/// Unicode replacement character
pub const REPLACEMENT: char = '\u{FFFD}';

/// Clear a screen region bounded by (clip.min.x,clip.min.y)..(clip.min.x,clip.max.y)
pub fn clear_region(fb: &mut FrBuf, clip: ClipRect) {
    if clip.max.y > LINES
        || clip.min.y >= clip.max.y
        || clip.max.x > WIDTH
        || clip.min.x >= clip.max.x
    {
        return;
    }
    // Calculate word alignment for destination buffer
    let dest_low_word = clip.min.x >> 5;
    let dest_high_word = clip.max.x >> 5;
    let px_in_dest_low_word = 32 - (clip.min.x & 0x1f);
    let px_in_dest_high_word = clip.max.x & 0x1f;
    // Blit it
    for y in clip.min.y..clip.max.y {
        let base = y * WORDS_PER_LINE;
        fb[base + dest_low_word] |= 0xffffffff << (32 - px_in_dest_low_word);
        for w in dest_low_word + 1..dest_high_word {
            fb[base + w] = 0xffffffff;
        }
        if dest_low_word < dest_high_word {
            fb[base + dest_high_word] |= 0xffffffff >> (32 - px_in_dest_high_word);
        }
    }
}

/// Find glyph for char using latin regular, emoji, ja, zh, and kr font data
pub fn find_glyph(ch: char) -> fonts::GlyphSprite {
    match fonts::regular_glyph(ch) {
        Ok(g) => g,
        _ => match fonts::emoji_glyph(ch) {
            Ok(g) => g,
            _ => match fonts::ja_glyph(ch) {
                Ok(g) => g,
                _ => match fonts::zh_glyph(ch) {
                    Ok(g) => g,
                    _ => match fonts::kr_glyph(ch) {
                        Ok(g) => g,
                        _ => match fonts::regular_glyph(REPLACEMENT) {
                            Ok(g) => g,
                            _ => NULL_GLYPH_SPRITE,
                        },
                    },
                },
            },
        },
    }
}

/// Find glyph for char using only the latin small font data
pub fn find_glyph_latin_small(ch: char) -> fonts::GlyphSprite {
    match fonts::small_glyph(ch) {
        Ok(g) => g,
        _ => match fonts::small_glyph(REPLACEMENT) {
            Ok(g) => g,
            _ => NULL_GLYPH_SPRITE,
        },
    }
}

/// Find glyph for char using only the latin regular font data
pub fn find_glyph_latin_regular(ch: char) -> fonts::GlyphSprite {
    match fonts::regular_glyph(ch) {
        Ok(g) => g,
        _ => match fonts::regular_glyph(REPLACEMENT) {
            Ok(g) => g,
            _ => NULL_GLYPH_SPRITE,
        },
    }
}

/// Find glyph for char using only the latin bold font data
pub fn find_glyph_latin_bold(ch: char) -> fonts::GlyphSprite {
    match fonts::bold_glyph(ch) {
        Ok(g) => g,
        _ => match fonts::bold_glyph(REPLACEMENT) {
            Ok(g) => g,
            _ => NULL_GLYPH_SPRITE,
        },
    }
}

/// Find glyph for char using only the latin mono font data
pub fn find_glyph_latin_mono(ch: char) -> fonts::GlyphSprite {
    match fonts::mono_glyph(ch) {
        Ok(g) => g,
        _ => match fonts::mono_glyph(REPLACEMENT) {
            Ok(g) => g,
            _ => NULL_GLYPH_SPRITE,
        },
    }
}

/// XOR blit a string using multi-lingual glyphs with specified clip rect, starting at cursor
pub fn paint_str(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 2;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph(ch);
            let wide = glyph.wide as usize;
            let high = glyph.high as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// XOR blit a 2x scaled string using multi-lingual glyphs with specified clip rect, starting at cursor
pub fn paint_str_2x(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 4;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph(ch);
            let wide = (glyph.wide << 1) as usize;
            let high = (glyph.high << 1) as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph_2x(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// XOR blit a string using latin small glyphs with specified clip rect, starting at cursor
pub fn paint_str_latin_small(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 1;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph_latin_small(ch);
            let wide = glyph.wide as usize;
            let high = glyph.high as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// XOR blit a string using latin small glyphs with 2x scaling and specified clip rect, starting at cursor
pub fn paint_str_latin_small_2x(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 2;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph_latin_small(ch);
            let wide = (glyph.wide << 1) as usize;
            let high = (glyph.high << 1) as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph_2x(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// XOR blit a string using latin regular glyphs with specified clip rect, starting at cursor
pub fn paint_str_latin_regular(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 2;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph_latin_regular(ch);
            let wide = glyph.wide as usize;
            let high = glyph.high as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// XOR blit a string using latin bold glyphs with specified clip rect, starting at cursor
pub fn paint_str_latin_bold(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 2;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph_latin_bold(ch);
            let wide = glyph.wide as usize;
            let high = glyph.high as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// XOR blit a string using latin mono glyphs with specified clip rect, starting at cursor
pub fn paint_str_latin_mono(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    const KERN: usize = 1;
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            let glyph = find_glyph_latin_mono(ch);
            let wide = glyph.wide as usize;
            let high = glyph.high as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + KERN >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph(fb, &c.pt, glyph);
            c.pt.x += wide + KERN;
            if high > c.line_height {
                c.line_height = high;
            }
        }
    }
}

/// Advance the cursor to the start of a new line within the clip rect
pub fn newline(clip: ClipRect, c: &mut Cursor) {
    c.pt.x = clip.min.x;
    if c.line_height < fonts::small::MAX_HEIGHT as usize {
        c.line_height = fonts::small::MAX_HEIGHT as usize;
    }
    c.pt.y += c.line_height + 2;
    c.line_height = 0;
}

/// Blit a glyph with XOR at point; caller is responsible for word wrap.
///
/// Examples of word alignment for destination frame buffer:
/// 1. Fits in word: xr:1..7   => (data[0].bit_30)->(data[0].bit_26), mask:0x7c00_0000
/// 2. Spans words:  xr:30..36 => (data[0].bit_01)->(data[1].bit_29), mask:[0x0000_0003,0xe000_000]
///
pub fn xor_glyph(fb: &mut FrBuf, p: &Pt, gs: fonts::GlyphSprite) {
    const SPRITE_PX: usize = 16;
    const SPRITE_WORDS: usize = 8;
    if gs.glyph.len() < SPRITE_WORDS {
        // Fail silently if the glyph slice was too small
        // TODO: Maybe return an error? Not sure which way is better.
        return;
    }
    let high = gs.high as usize;
    let wide = gs.wide as usize;
    if high > SPRITE_PX || wide > SPRITE_PX {
        // Fail silently if glyph height or width is out of spec
        // TODO: Maybe return an error?
        return;
    }
    // Calculate word alignment for destination buffer
    let x0 = p.x;
    let x1 = p.x + wide - 1;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it (use glyph height to avoid blitting empty rows)
    let mut row_base = p.y * WORDS_PER_LINE;
    const ROW_LIMIT: usize = LINES * WORDS_PER_LINE;
    let glyph = gs.glyph;
    for y in 0..high {
        if row_base >= ROW_LIMIT {
            // Clip anything that would run off the end of the frame buffer
            break;
        }
        // Unpack pixels for this glyph row.
        // CAUTION: some math magic happening here...
        //  when y==0, this does (glyph[0] >>  0) & mask,
        //  when y==1, this does (glyph[0] >> 16) & mask,
        //  when y==2, this does (glyph[1] >>  0) & mask,
        //  ...
        let mask = 0x0000ffff as u32;
        let shift = (y & 1) << 4;
        let pattern = (glyph[y >> 1] >> shift) & mask;
        // XOR glyph pixels onto destination buffer
        fb[row_base + dest_low_word] ^= pattern << (32 - px_in_dest_low_word);
        if wide > px_in_dest_low_word {
            fb[row_base + dest_high_word] ^= pattern >> px_in_dest_low_word;
        }
        // Advance destination offset using + instead of * to maybe save some CPU cycles
        row_base += WORDS_PER_LINE;
    }
}

/// Blit a 2x scaled glyph with XOR at point; caller is responsible for word wrap.
///
/// This is similar to xor_glyph(). But, instead of using 16px sprites for input
/// and output, this takes 16px sprites as input and blits 32px sprites as output.
///
pub fn xor_glyph_2x(fb: &mut FrBuf, p: &Pt, gs: fonts::GlyphSprite) {
    const SPRITE_PX: usize = 16;
    const SPRITE_WORDS: usize = 8;
    if gs.glyph.len() < SPRITE_WORDS {
        // Fail silently if the glyph slice was too small
        // TODO: Maybe return an error? Not sure which way is better.
        return;
    }
    let high = gs.high as usize;
    let wide = gs.wide as usize;
    if high > SPRITE_PX || wide > SPRITE_PX {
        // Fail silently if glyph height or width is out of spec
        // TODO: Maybe return an error?
        return;
    }
    // Calculate word alignment for destination buffer
    let x0 = p.x;
    let x1 = p.x + (wide << 1) - 1;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it (use glyph height to avoid blitting empty rows)
    let mut row_base = p.y * WORDS_PER_LINE;
    const ROW_LIMIT: usize = LINES * WORDS_PER_LINE;
    let glyph = gs.glyph;
    // Scale up 2x
    let mut glyph_2x = [0u32; 32];
    let mask = 0x0000ffff as u32;
    for y in 0..high {
        let shift = (y & 1) << 4;
        let pattern = (glyph[y >> 1] >> shift) & mask;
        let low_2x = LUT_2X[(pattern & 0xff) as usize] as u32;
        let high_2x = LUT_2X[((pattern >> 8) & 0xff) as usize] as u32;
        let both_2x = low_2x | (high_2x << 16);
        let index_2x = y << 1;
        glyph_2x[index_2x] = both_2x;
        glyph_2x[index_2x + 1] = both_2x;
    }
    // Blit the scaled up glyph
    for src in glyph_2x {
        if row_base >= ROW_LIMIT {
            // Clip anything that would run off the end of the frame buffer
            break;
        }
        // XOR glyph pixels onto destination buffer
        fb[row_base + dest_low_word] ^= src << (32 - px_in_dest_low_word);
        if (wide << 1) > px_in_dest_low_word {
            fb[row_base + dest_high_word] ^= src >> px_in_dest_low_word;
        }
        // Advance destination offset using + instead of * to maybe save some CPU cycles
        row_base += WORDS_PER_LINE;
    }
}

/// Lookup table to speed up 2x scaling by expanding u8 index to u16 value
pub const LUT_2X: [u16; 256] = [
    0b0000000000000000,
    0b0000000000000011,
    0b0000000000001100,
    0b0000000000001111,
    0b0000000000110000,
    0b0000000000110011,
    0b0000000000111100,
    0b0000000000111111,
    0b0000000011000000,
    0b0000000011000011,
    0b0000000011001100,
    0b0000000011001111,
    0b0000000011110000,
    0b0000000011110011,
    0b0000000011111100,
    0b0000000011111111,
    0b0000001100000000,
    0b0000001100000011,
    0b0000001100001100,
    0b0000001100001111,
    0b0000001100110000,
    0b0000001100110011,
    0b0000001100111100,
    0b0000001100111111,
    0b0000001111000000,
    0b0000001111000011,
    0b0000001111001100,
    0b0000001111001111,
    0b0000001111110000,
    0b0000001111110011,
    0b0000001111111100,
    0b0000001111111111,
    0b0000110000000000,
    0b0000110000000011,
    0b0000110000001100,
    0b0000110000001111,
    0b0000110000110000,
    0b0000110000110011,
    0b0000110000111100,
    0b0000110000111111,
    0b0000110011000000,
    0b0000110011000011,
    0b0000110011001100,
    0b0000110011001111,
    0b0000110011110000,
    0b0000110011110011,
    0b0000110011111100,
    0b0000110011111111,
    0b0000111100000000,
    0b0000111100000011,
    0b0000111100001100,
    0b0000111100001111,
    0b0000111100110000,
    0b0000111100110011,
    0b0000111100111100,
    0b0000111100111111,
    0b0000111111000000,
    0b0000111111000011,
    0b0000111111001100,
    0b0000111111001111,
    0b0000111111110000,
    0b0000111111110011,
    0b0000111111111100,
    0b0000111111111111,
    0b0011000000000000,
    0b0011000000000011,
    0b0011000000001100,
    0b0011000000001111,
    0b0011000000110000,
    0b0011000000110011,
    0b0011000000111100,
    0b0011000000111111,
    0b0011000011000000,
    0b0011000011000011,
    0b0011000011001100,
    0b0011000011001111,
    0b0011000011110000,
    0b0011000011110011,
    0b0011000011111100,
    0b0011000011111111,
    0b0011001100000000,
    0b0011001100000011,
    0b0011001100001100,
    0b0011001100001111,
    0b0011001100110000,
    0b0011001100110011,
    0b0011001100111100,
    0b0011001100111111,
    0b0011001111000000,
    0b0011001111000011,
    0b0011001111001100,
    0b0011001111001111,
    0b0011001111110000,
    0b0011001111110011,
    0b0011001111111100,
    0b0011001111111111,
    0b0011110000000000,
    0b0011110000000011,
    0b0011110000001100,
    0b0011110000001111,
    0b0011110000110000,
    0b0011110000110011,
    0b0011110000111100,
    0b0011110000111111,
    0b0011110011000000,
    0b0011110011000011,
    0b0011110011001100,
    0b0011110011001111,
    0b0011110011110000,
    0b0011110011110011,
    0b0011110011111100,
    0b0011110011111111,
    0b0011111100000000,
    0b0011111100000011,
    0b0011111100001100,
    0b0011111100001111,
    0b0011111100110000,
    0b0011111100110011,
    0b0011111100111100,
    0b0011111100111111,
    0b0011111111000000,
    0b0011111111000011,
    0b0011111111001100,
    0b0011111111001111,
    0b0011111111110000,
    0b0011111111110011,
    0b0011111111111100,
    0b0011111111111111,
    0b1100000000000000,
    0b1100000000000011,
    0b1100000000001100,
    0b1100000000001111,
    0b1100000000110000,
    0b1100000000110011,
    0b1100000000111100,
    0b1100000000111111,
    0b1100000011000000,
    0b1100000011000011,
    0b1100000011001100,
    0b1100000011001111,
    0b1100000011110000,
    0b1100000011110011,
    0b1100000011111100,
    0b1100000011111111,
    0b1100001100000000,
    0b1100001100000011,
    0b1100001100001100,
    0b1100001100001111,
    0b1100001100110000,
    0b1100001100110011,
    0b1100001100111100,
    0b1100001100111111,
    0b1100001111000000,
    0b1100001111000011,
    0b1100001111001100,
    0b1100001111001111,
    0b1100001111110000,
    0b1100001111110011,
    0b1100001111111100,
    0b1100001111111111,
    0b1100110000000000,
    0b1100110000000011,
    0b1100110000001100,
    0b1100110000001111,
    0b1100110000110000,
    0b1100110000110011,
    0b1100110000111100,
    0b1100110000111111,
    0b1100110011000000,
    0b1100110011000011,
    0b1100110011001100,
    0b1100110011001111,
    0b1100110011110000,
    0b1100110011110011,
    0b1100110011111100,
    0b1100110011111111,
    0b1100111100000000,
    0b1100111100000011,
    0b1100111100001100,
    0b1100111100001111,
    0b1100111100110000,
    0b1100111100110011,
    0b1100111100111100,
    0b1100111100111111,
    0b1100111111000000,
    0b1100111111000011,
    0b1100111111001100,
    0b1100111111001111,
    0b1100111111110000,
    0b1100111111110011,
    0b1100111111111100,
    0b1100111111111111,
    0b1111000000000000,
    0b1111000000000011,
    0b1111000000001100,
    0b1111000000001111,
    0b1111000000110000,
    0b1111000000110011,
    0b1111000000111100,
    0b1111000000111111,
    0b1111000011000000,
    0b1111000011000011,
    0b1111000011001100,
    0b1111000011001111,
    0b1111000011110000,
    0b1111000011110011,
    0b1111000011111100,
    0b1111000011111111,
    0b1111001100000000,
    0b1111001100000011,
    0b1111001100001100,
    0b1111001100001111,
    0b1111001100110000,
    0b1111001100110011,
    0b1111001100111100,
    0b1111001100111111,
    0b1111001111000000,
    0b1111001111000011,
    0b1111001111001100,
    0b1111001111001111,
    0b1111001111110000,
    0b1111001111110011,
    0b1111001111111100,
    0b1111001111111111,
    0b1111110000000000,
    0b1111110000000011,
    0b1111110000001100,
    0b1111110000001111,
    0b1111110000110000,
    0b1111110000110011,
    0b1111110000111100,
    0b1111110000111111,
    0b1111110011000000,
    0b1111110011000011,
    0b1111110011001100,
    0b1111110011001111,
    0b1111110011110000,
    0b1111110011110011,
    0b1111110011111100,
    0b1111110011111111,
    0b1111111100000000,
    0b1111111100000011,
    0b1111111100001100,
    0b1111111100001111,
    0b1111111100110000,
    0b1111111100110011,
    0b1111111100111100,
    0b1111111100111111,
    0b1111111111000000,
    0b1111111111000011,
    0b1111111111001100,
    0b1111111111001111,
    0b1111111111110000,
    0b1111111111110011,
    0b1111111111111100,
    0b1111111111111111,
];
