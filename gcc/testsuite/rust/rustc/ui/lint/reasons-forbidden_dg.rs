// If you turn off deduplicate diagnostics (which rustc turns on by default but
// compiletest turns off when it runs ui tests), then the errors are
// (unfortunately) repeated here because the checking is done as we read in the
// errors, and currently that happens two or three different times, depending on
// compiler flags.
//
// The test is much cleaner if we deduplicate, though.

//@ compile-flags: -Z deduplicate-diagnostics=true

#![forbid(
    unsafe_code,
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    reason = "our errors & omissions insurance policy doesn't cover unsafe Rust"
)]

use std::ptr;

fn main() {
    let a_billion_dollar_mistake = ptr::null();

    #[allow(unsafe_code)]
// { dg-error ".E0453." "" { target *-*-* } .-1 }
// { dg-note ".E0453." "" { target *-*-* } .-2 }
// { dg-note ".E0453." "" { target *-*-* } .-3 }
    unsafe {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        *a_billion_dollar_mistake
    }
}

