//@ needs-asm-support

use std::arch::asm;

const _: () = unsafe { asm!("nop") };
// { dg-error ".E0015." "" { target *-*-* } .-1 }

fn main() {}

