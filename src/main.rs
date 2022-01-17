// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use blitstr2::demo;

/// This runs the demo with no visible output, which is mostly only useful for
/// debugging changes to the blitting code. Since main() links with the standard
/// library, any panics will get printed (unlike with wasm).
fn main() {
    // Allocate frame buffer
    let fb = &mut blitstr2::new_fr_buf();

    // Call painting code
    demo::sample_text(fb);

    // Unimplemented: copy frame buffer to a display device
}
