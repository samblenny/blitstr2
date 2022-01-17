// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use super::{clear_region, paint_str, ClipRect, Cursor, FrBuf};

/// Demonstrate available fonts
pub fn sample_text(fb: &mut FrBuf) {
    let note = &"Hello, world! ää 🀄🃏\u{1F170}\u{1F170}\u{FE0F}\n"; // This has Unicode NFC and NFD
    let sas1 = &"\n   🍎       🎸       🕶        🍎\n";
    let sas2 = &"  apple     guitar    glasses     apple\n\n";
    let sas3 = &"           😸     🎩    🔑\n";
    let sas4 = &"           cat    hat    key\n\n";
    let wrap = &concat!(
        "The quick brown fox jumps over the lazy dog.\n\n",
        "Zwölf Boxkämpfer jagen Viktor quer über den\n großen Sylter Deich.\n\n"
    );
    let iroha = &concat!(
        "いろはにほへと\nちりぬるを\nわかよたれそ\nつねならむ\n",
        "うゐのおくやま\nけふこえて\nあさきゆめみし\nゑひもせす\n\n",
    );
    let goose = &concat!(
        "鹅、鹅、鹅，\n",
        "曲项向天歌。\n",
        "白毛浮绿水，\n",
        "红掌拨清波\n\n",
    );
    let coffee = &"커피 주세요\n";
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
