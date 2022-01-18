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
///
/// Rationale and Procedures:
///
/// Many of these tests call a function from crate::demo that modifies a
/// frame buffer, then verify that the murmur3 hash of the frame buffer
/// contents match an expected value. The idea is to maintain pixel-accurate
/// consistency in blitting glyphs. If something about the glyph blitting or
/// string rendering changes, a frame buffer hash should also change.
///
/// To update the frame buffer hashes, the procedure is:
/// 1. Modify wasm_demo/src/lib.rs::init() to call the demo function
/// 2. Rebuild the wasm_demo: cd wasm_demo; make install; ./webserver.rb
/// 3. Load the wasm_demo in a browser (http://localhost:8000) and verify
///    that the demo text looks as it should
/// 4. Run cargo test: the test that calls the demo function should be
///    failing the assert that checks the frame buffer. Note the left-hand
///    hash value from the error message
/// 5. Edit the assert in the test function to have the new hash value
///
#[cfg(test)]
mod tests {
    use crate::blit::clear_region;
    use crate::cliprect::ClipRect;
    use crate::cursor::Cursor;
    use crate::demo;
    use crate::framebuffer::{new_fr_buf, FrBuf, FRAME_BUF_SIZE, LINES, WIDTH, WORDS_PER_LINE};
    use crate::m3hash;
    use crate::pt::Pt;

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
    /// This covers several fonts:
    ///  - latin regular
    ///  - emoji
    ///  - Japanese
    ///  - Simplified Chinese
    ///  - Korean
    fn test_demo_sample_text_frame_buffer_hash() {
        let fb = &mut new_fr_buf();
        demo::sample_text(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 770682562);
        assert_eq!(m3hash::frame_buffer(fb, 1), 1047968722);
    }

    #[test]
    /// Test that the small latin font works
    fn test_paint_latin_small_sampler() {
        let fb = &mut new_fr_buf();
        demo::paint_latin_small_sampler(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 4022162394);
    }

    #[test]
    /// Test that the regular latin font works
    fn test_paint_latin_regular_sampler() {
        let fb = &mut new_fr_buf();
        demo::paint_latin_regular_sampler(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 1881526323);
    }

    #[test]
    /// Test that the bold latin font works
    fn test_paint_latin_bold_sampler() {
        let fb = &mut new_fr_buf();
        demo::paint_latin_bold_sampler(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 2068748088);
    }

    #[test]
    /// Test that the monospace latin font works
    fn test_paint_latin_mono_sampler() {
        let fb = &mut new_fr_buf();
        demo::paint_latin_mono_sampler(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 1438437699);
    }

    #[test]
    /// This tests that word-wrapping and text layout work properly. It should
    /// be possible to get the exact same results by using one paint_str()
    /// call to paint a full string, or by reusing a cursor across many calls
    /// to paint_str() to paint the string character by character.
    fn test_paint_str_full_string_vs_char_by_char() {
        let fb = &mut new_fr_buf();
        demo::paint_pangram_as_full_str(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 229943020); // Same hash
        demo::paint_pangram_char_by_char(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 229943020); // Same hash
    }

    #[test]
    /// This checks raw glyph blitting without word wrapping
    fn test_low_level_glyph_blits() {
        let fb = &mut new_fr_buf();
        demo::low_level_glyph_blits(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 2300273120);
    }

    #[test]
    /// ClipRect is used for word-wrapping
    fn test_cliprect() {
        let cr1 = ClipRect {
            min: Pt { x: 1, y: 2 },
            max: Pt { x: 3, y: 4 },
        };
        assert_eq!(cr1, ClipRect::new(1, 2, 3, 4));
        assert_ne!(ClipRect::full_screen(), ClipRect::padded_screen());
    }

    #[test]
    /// Cursor is used for text layout and word-wrapping
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
    /// FrBuf is the frame buffer target of blit operations
    fn test_framebuffer() {
        assert_eq!(LINES * WORDS_PER_LINE, FRAME_BUF_SIZE);
        assert!(LINES > 0);
        assert!(WIDTH > 0);
        let fb: FrBuf = new_fr_buf();
        assert!(fb.len() > 0);
    }

    #[test]
    /// Pt is used to track the point at which to blit a glyph
    fn test_pt() {
        let p1 = Pt { x: 1, y: 2 };
        let p2 = Pt::new(1, 2);
        assert_eq!(p1, p2);
    }
}
