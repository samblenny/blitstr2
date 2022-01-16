// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

/// Panic Handler for no_std.
use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic_info: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}
