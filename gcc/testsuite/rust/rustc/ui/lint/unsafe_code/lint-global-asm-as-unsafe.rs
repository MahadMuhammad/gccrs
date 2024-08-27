//@ needs-asm-support
#![deny(unsafe_code)]

use std::arch::global_asm;

#[allow(unsafe_code)]
mod allowed_unsafe {
    std::arch::global_asm!("");
}

macro_rules! unsafe_in_macro {
    () => {
        global_asm!(""); // { dg-error "" "" { target *-*-* } }
    };
}

global_asm!(""); // { dg-error "" "" { target *-*-* } }
unsafe_in_macro!();

fn main() {}

