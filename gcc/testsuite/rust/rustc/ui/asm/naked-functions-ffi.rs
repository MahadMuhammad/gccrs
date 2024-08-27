//@ check-pass
//@ needs-asm-support
#![feature(naked_functions)]
#![crate_type = "lib"]

use std::arch::asm;

#[naked]
pub extern "C" fn naked(p: char) -> u128 {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    unsafe {
        asm!("", options(noreturn));
    }
}

