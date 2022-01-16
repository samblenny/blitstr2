// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use crate::cliprect::ClipRect;
use crate::cursor::Cursor;
use crate::fonts;
use crate::framebuffer::{FrBuf, LINES, WIDTH, WORDS_PER_LINE};

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
            // TODO: handle possible error for missing glyphs
            let _ = xor_char(fb, clip, c, ch);
        }
    }
}

/// Advance the cursor to the start of a new line within the clip rect
fn newline(clip: ClipRect, c: &mut Cursor) {
    c.pt.x = clip.min.x;
    if c.line_height < fonts::small::MAX_HEIGHT as usize {
        c.line_height = fonts::small::MAX_HEIGHT as usize;
    }
    c.pt.y += c.line_height + 1;
    c.line_height = 0;
}

/// Blit a char with: XOR, align left:xr.0 top:yr.0, pad L:1px R:2px
/// Return: width in pixels of character + padding that were blitted (0 if won't fit in clip region)
///
/// Examples of word alignment for destination frame buffer:
/// 1. Fits in word: xr:1..7   => (data[0].bit_30)->(data[0].bit_26), mask:0x7c00_0000
/// 2. Spans words:  xr:30..36 => (data[0].bit_01)->(data[1].bit_29), mask:[0x0000_0003,0xe000_000]
///
fn xor_char(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, ch: char) -> Result<(), usize> {
    if clip.max.y > LINES || clip.max.x > WIDTH || clip.min.x >= clip.max.x {
        return Ok(());
    }
    // Look up glyph for character and unpack its header
    // TODO: make this aware of multiple fonts
    let glyph = match fonts::regular_glyph(ch) {
        Ok(g) => g,
        _ => fonts::emoji_glyph(ch)?,
    };
    // TODO: determine actual width for proportional fonts
    let wide = 16;
    // TODO: make this aware of multiple fonts
    let high = 16 as usize;
    // Don't clip if cursor is left of clip rect; instead, advance the cursor
    if c.pt.x < clip.min.x {
        c.pt.x = clip.min.x;
    }
    // Add 1px pad to left
    let mut x0 = c.pt.x + 1;
    // Adjust for word wrapping
    if x0 + wide + 2 >= clip.max.x {
        newline(clip, c);
        x0 = c.pt.x + 1;
    }
    // Calculate word alignment for destination buffer
    let x1 = x0 + wide;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it
    let y0 = c.pt.y;
    if y0 > clip.max.y {
        return Ok(()); // Entire glyph is outside clip rect, so clip it
    }
    let y_max = if (y0 + high) <= clip.max.y {
        high
    } else {
        clip.max.y - y0 // Clip bottom of glyph
    };
    for y in 0..y_max {
        // Skip rows that are above the clip region
        if y0 + y < clip.min.y {
            continue; // Clip top of glyph
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
        let base = (y0 + y) * WORDS_PER_LINE;
        fb[base + dest_low_word] ^= pattern << (32 - px_in_dest_low_word);
        fb[base + dest_high_word] ^= pattern >> px_in_dest_low_word;
    }
    let width_of_blitted_pixels = wide + 3;
    c.pt.x += width_of_blitted_pixels;
    if high > c.line_height {
        c.line_height = high;
    }
    return Ok(());
}
