// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use crate::cliprect::ClipRect;
use crate::cursor::Cursor;
use crate::fonts;
use crate::framebuffer::{FrBuf, LINES, WIDTH, WORDS_PER_LINE};
use crate::pt::Pt;

/// Null glyph to use when everything else fails
const NULL_GLYPH: [u32; 8] = [0, 0x5500AA, 0x5500AA, 0x5500AA, 0x5500AA, 0x5500AA, 0, 0];
const NULL_GLYPH_SPRITE: fonts::GlyphSprite = fonts::GlyphSprite {
    glyph: &NULL_GLYPH,
    wide: 8u8,
    high: 12u8,
};

/// Unicode replacement character
const REPLACEMENT: char = '\u{FFFD}';

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

/// XOR blit a string with specified style, clip rect, starting at cursor
pub fn paint_str(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, s: &str) {
    for ch in s.chars() {
        if ch == '\n' {
            newline(clip, c);
        } else {
            // Look up the glyph for this char
            // TODO: make this better for failover between multiple fonts
            let glyph = match fonts::regular_glyph(ch) {
                Ok(g) => g,
                _ => match fonts::emoji_glyph(ch) {
                    Ok(g) => g,
                    _ => match fonts::regular_glyph(REPLACEMENT) {
                        Ok(g) => g,
                        _ => NULL_GLYPH_SPRITE,
                    },
                },
            };
            // TODO: determine actual width for proportional fonts
            let wide = glyph.wide as usize;
            // TODO: make this aware of multiple fonts
            let high = glyph.high as usize;
            // Adjust for word wrapping
            if c.pt.x + wide + 2 >= clip.max.x {
                newline(clip, c);
            }
            // Blit the glyph and advance the cursor
            xor_glyph(fb, &c.pt, glyph);
            c.pt.x += wide + 2;
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
    c.pt.y += c.line_height + 1;
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
