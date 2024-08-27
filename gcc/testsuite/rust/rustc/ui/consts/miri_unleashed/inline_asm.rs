//@ compile-flags: -Zunleash-the-miri-inside-of-you
//@ only-x86_64

use std::arch::asm;

fn main() {}

// Make sure we catch executing inline assembly.
static TEST_BAD: () = {
    unsafe { asm!("nop"); }
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-note ".E0080." "" { target *-*-* } .-2 }
};

