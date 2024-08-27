//@ needs-asm-support

use std::arch::asm;

fn main() {
    asm!("nop"); // { dg-error ".E0133." "" { target *-*-* } }
}

