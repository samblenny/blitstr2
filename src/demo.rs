// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use crate::blit::{
    clear_region, paint_str, paint_str_2x, paint_str_latin_bold, paint_str_latin_mono,
    paint_str_latin_regular, paint_str_latin_small, paint_str_latin_small_2x, xor_glyph,
};
use crate::cliprect::ClipRect;
use crate::cursor::Cursor;
use crate::fonts::{
    bold, emoji_glyph, ja_glyph, kr_glyph, mono, regular, regular_glyph, small, small_glyph,
    zh_glyph,
};
use crate::framebuffer::FrBuf;
use crate::pt::Pt;

/// Demonstrate available fonts
pub fn sample_text(fb: &mut FrBuf) {
    let note = &"Hello, world! Ã¤aÌ ðð\u{1F170}\u{1F170}\u{FE0F}\n"; // This has Unicode NFC and NFD
    let sas1 = &"\n   ð       ð¸       ð¶        ð\n";
    let sas2 = &"  apple     guitar    glasses     apple\n\n";
    let sas3 = &"           ð¸     ð©    ð\n";
    let sas4 = &"           cat    hat    key\n\n";
    let wrap = &concat!(
        "The quick brown fox jumps over the lazy dog.\n\n",
        "ZwÃ¶lf BoxkÃ¤mpfer jagen Viktor quer Ã¼ber den\n groÃen Sylter Deich.\n\n"
    );
    let iroha = &concat!(
        "ããã¯ã«ã»ã¸ã¨\nã¡ãã¬ãã\nãããããã\nã¤ã­ãªãã\n",
        "ããã®ãããã¾\nããµããã¦\nãããããã¿ã\nãã²ããã\n\n",
    );
    let goose = &concat!(
        "é¹ãé¹ãé¹ï¼\n",
        "æ²é¡¹åå¤©æ­ã\n",
        "ç½æ¯æµ®ç»¿æ°´ï¼\n",
        "çº¢ææ¨æ¸æ³¢\n\n",
    );
    let coffee = &"ì»¤í¼ ì£¼ì¸ì\n";
    clear_region(fb, ClipRect::full_screen());
    let clip = ClipRect::padded_screen();
    let c = &mut Cursor::from_top_left_of(clip);
    paint_str(fb, clip, c, note);
    paint_str(fb, clip, c, sas1);
    paint_str(fb, clip, c, sas2);
    paint_str(fb, clip, c, sas3);
    paint_str(fb, clip, c, sas4);
    paint_str(fb, clip, c, wrap);
    paint_str(fb, clip, c, iroha);
    paint_str(fb, clip, c, goose);
    paint_str(fb, clip, c, coffee);
}

/// Demonstrate available fonts with 2x scaling
pub fn sample_text_2x(fb: &mut FrBuf) {
    let wrap = &concat!(
        " ð ð¸ ð¶  ð ð¸ ð©  ð\n",
        "The quick brown fox jumps over the lazy dog.\n",
        "ZwÃ¶lf BoxkÃ¤mpfer jagen Viktor quer Ã¼ber den\n groÃen Sylter Deich.\n",
        "ããã¯ã«ã»ã¸ã¨\n",
        "é¹ãé¹ãé¹ï¼\n",
        "ì»¤í¼ ì£¼ì¸ì"
    );
    clear_region(fb, ClipRect::full_screen());
    let clip = ClipRect::padded_screen();
    let c = &mut Cursor::from_top_left_of(clip);
    paint_str_2x(fb, clip, c, wrap);
}

pub const PANGRAM: &str = "The quick brown fox jumps over the lazy dog.";

/// Paint pangram all at once
pub fn paint_pangram_as_full_str(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    paint_str(fb, clip, cursor, PANGRAM);
}

/// Paint pangram char by char (result should match all at once above)
pub fn paint_pangram_char_by_char(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    for i in 0..PANGRAM.len() {
        // This slicing is sort of like &str.iter(), but I needed a thing to
        // yield &str instead of char, because paint_str() is designed for &str
        if let Some((j, _)) = PANGRAM.char_indices().nth(i) {
            let c = match PANGRAM.char_indices().nth(j + 1) {
                Some((k, _)) => &PANGRAM[j..k],
                _ => &PANGRAM[j..],
            };
            paint_str(fb, clip, cursor, c);
        } else {
            break; // That was the last char, so stop now
        }
    }
}

/// Paint sampler in small latin glyphs
pub fn paint_latin_small_sampler(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    // First, iterate over every glyph in the font
    let mut buf = [0u8; 4];
    for cp in small::CODEPOINTS {
        match char::from_u32(cp) {
            Some(c) => {
                let s = c.encode_utf8(&mut buf);
                paint_str_latin_small(fb, clip, cursor, s)
            }
            _ => (),
        };
    }
    // Then, do the pangram
    paint_str_latin_small(fb, clip, cursor, &"\n\n");
    paint_str_latin_small(fb, clip, cursor, PANGRAM);
}

/// Paint sampler in small latin glyphs with 2x scale (9px becomes 18px)
pub fn paint_latin_small_sampler_2x(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    // First, iterate over every glyph in the font
    let mut buf = [0u8; 4];
    for cp in small::CODEPOINTS {
        match char::from_u32(cp) {
            Some(c) => {
                let s = c.encode_utf8(&mut buf);
                paint_str_latin_small_2x(fb, clip, cursor, s)
            }
            _ => (),
        };
    }
    // Then, do the pangram
    paint_str_latin_small_2x(fb, clip, cursor, &"\n\n");
    paint_str_latin_small_2x(fb, clip, cursor, PANGRAM);
}

/// Paint sampler in regular latin glyphs
pub fn paint_latin_regular_sampler(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    // First, iterate over every glyph in the font
    let mut buf = [0u8; 4];
    for cp in regular::CODEPOINTS {
        match char::from_u32(cp) {
            Some(c) => {
                let s = c.encode_utf8(&mut buf);
                paint_str_latin_regular(fb, clip, cursor, s)
            }
            _ => (),
        };
    }
    // Then, do the pangram
    paint_str_latin_regular(fb, clip, cursor, &"\n\n");
    paint_str_latin_regular(fb, clip, cursor, PANGRAM);
}

/// Paint sampler in latin bold glyphs
pub fn paint_latin_bold_sampler(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    let mut buf = [0u8; 4];
    // First, iterate over every glyph in the font
    for cp in bold::CODEPOINTS {
        match char::from_u32(cp) {
            Some(c) => {
                let s = c.encode_utf8(&mut buf);
                paint_str_latin_bold(fb, clip, cursor, s)
            }
            _ => (),
        };
    }
    // Then, do the pangram
    paint_str_latin_bold(fb, clip, cursor, &"\n\n");
    paint_str_latin_bold(fb, clip, cursor, PANGRAM);
}

/// Paint sampler in latin mono glyphs
pub fn paint_latin_mono_sampler(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let cursor = &mut Cursor::from_top_left_of(clip);
    let mut buf = [0u8; 4];
    // First, iterate over every glyph in the font
    for cp in mono::CODEPOINTS {
        match char::from_u32(cp) {
            Some(c) => {
                let s = c.encode_utf8(&mut buf);
                paint_str_latin_mono(fb, clip, cursor, s)
            }
            _ => (),
        };
    }
    // Then, do the pangram
    paint_str_latin_mono(fb, clip, cursor, &"\n\n");
    paint_str_latin_mono(fb, clip, cursor, PANGRAM);
}

/// Do low-level glyph blitting without word-wrapping
pub fn low_level_glyph_blits(fb: &mut FrBuf) {
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);
    let mut pt = Pt { x: 5, y: 5 };
    let mut gs = small_glyph('M').unwrap();
    xor_glyph(fb, &pt, gs);
    pt.x += 20;
    gs = regular_glyph('M').unwrap();
    xor_glyph(fb, &pt, gs);
    pt.x += 20;
    gs = emoji_glyph('ð¸').unwrap();
    xor_glyph(fb, &pt, gs);
    pt.x += 20;
    gs = zh_glyph('é¹').unwrap();
    xor_glyph(fb, &pt, gs);
    pt.x += 20;
    gs = ja_glyph('ã').unwrap();
    xor_glyph(fb, &pt, gs);
    pt.x += 20;
    gs = kr_glyph('ì»¤').unwrap();
    xor_glyph(fb, &pt, gs);
}
