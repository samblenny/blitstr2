// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]

// Just export all the things so people can mix and match building blocks
// as they like. In particular, this is meant to allow for flexible DIY
// word-wrapping and glyph-lookup strategies beyond what I've implemented
// here already.
pub mod blit;
pub mod cliprect;
pub mod cursor;
pub mod demo;
pub mod fonts;
pub mod framebuffer;
pub mod m3hash;
pub mod pt;

/// These are integration tests aimed at ensuring pixel-accurate stability of
/// string painting operations by exercising edge cases around glyph lookup,
/// word-wrapping, etc.
#[cfg(test)]
mod tests {
    use crate::blit::{clear_region, paint_str};
    use crate::cliprect::ClipRect;
    use crate::cursor::Cursor;
    use crate::framebuffer::{new_fr_buf, FrBuf, FRAME_BUF_SIZE, LINES, WIDTH, WORDS_PER_LINE};
    use crate::m3hash;
    use crate::pt::Pt;
    use crate::demo;

    #[test]
    fn test_clear_region() {
        let fb = &mut new_fr_buf();
        clear_region(fb, ClipRect::full_screen());
        let seed = 0;
        assert_eq!(m3hash::frame_buffer(fb, seed), 0x3A25F08C);
    }

    #[test]
    /// Test for hashed frame buffer match using the font sampler demo screen.
    /// This covers many string blitting features and edge cases all at once.
    /// If this test fails, try loading the wasm demo to look for what changed.
    fn test_demo_sample_text_frame_buffer_hash() {
        let fb = &mut new_fr_buf();
        demo::sample_text(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x59AA26A1);
        assert_eq!(m3hash::frame_buffer(fb, 1), 0xAE37C33B);
    }

    #[test]
    /// Test paint_str() with GlyphStyle::Small and short ascii string
    fn test_paint_str_glyphstyle_small_abc() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, "abc");
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x5DE65BFC);
    }

    #[test]
    /// Test paint_str() with an emoji cat.
    fn test_paint_str_emoji_cat_multi_style() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, "😸");
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x3A4FFDB5); // Same hash
    }

    #[test]
    /// Test paint_str() for full string at once vs. concatenating chars.
    /// The point of this is, you can call paint_str() repeatedly reusing the
    /// same cursor, and it will keep track of concatenation and word-wrap.
    fn test_paint_str_full_string_vs_char_by_char() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        let s = "The quick brown fox jumps over the lazy dog.";
        // Paint the whole string at once
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, s);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0xE5240DD1); // Same hash

        // Paint it again one char at a time
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        for i in 0..s.len() {
            // This slicing is sort of like &str.iter(), but I needed a thing to
            // yield &str instead of char, because paint_str() is designed to
            // take grapheme clusters that can be more than 1 char long.
            if let Some((j, _)) = s.char_indices().nth(i) {
                let c = match s.char_indices().nth(j + 1) {
                    Some((k, _)) => &s[j..k],
                    _ => &s[j..],
                };
                paint_str(fb, clip, cursor, c);
            } else {
                break; // That was the last char, so stop now
            }
        }
        assert_eq!(m3hash::frame_buffer(fb, 0), 0xE5240DD1); // Same hash
    }

    #[test]
    fn test_blit() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, "abc");
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x529828DB);
    }

    #[test]
    fn test_cliprect() {
        let cr1 = ClipRect {
            min: Pt { x: 1, y: 2 },
            max: Pt { x: 3, y: 4 },
        };
        assert_eq!(cr1, ClipRect::new(1, 2, 3, 4));
        assert_ne!(ClipRect::full_screen(), ClipRect::padded_screen());
    }

    #[test]
    fn test_cursor() {
        let c1 = Cursor {
            pt: Pt { x: 1, y: 2 },
            line_height: 0,
        };
        assert_eq!(c1, Cursor::new(1, 2, 0));
        let clip = ClipRect::new(1, 2, 3, 4);
        let c2 = Cursor::from_top_left_of(clip);
        assert_eq!(c1.line_height, c2.line_height);
    }

    #[test]
    fn test_demo() {
        let fb = &mut new_fr_buf();
        demo::sample_text(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x59AA26A1);
    }

    #[test]
    fn test_framebuffer() {
        assert_eq!(LINES * WORDS_PER_LINE, FRAME_BUF_SIZE);
        assert!(LINES > 0);
        assert!(WIDTH > 0);
        let fb: FrBuf = new_fr_buf();
        assert!(fb.len() > 0);
    }

    #[test]
    fn test_pt() {
        let p1 = Pt { x: 1, y: 2 };
        let p2 = Pt::new(1, 2);
        assert_eq!(p1, p2);
    }
}
