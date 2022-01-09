// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
extern crate blitstr2;

use blitstr2::FrBuf;

static mut FB: FrBuf = blitstr2::new_fr_buf();

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

/// Initialize screen
#[no_mangle]
pub extern "C" fn init() {
    // Show sample text
    blitstr2::demo::sample_text(unsafe { &mut FB });
    blitstr2::demo::goose_poem(unsafe { &mut FB });
}

/// Export pointer to frame buffer shared memory for javascript + wasm32
#[no_mangle]
pub extern "C" fn frame_buf_ptr() -> *const u32 {
    unsafe { FB.as_ptr() }
}
