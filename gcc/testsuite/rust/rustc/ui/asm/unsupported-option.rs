//@ needs-asm-support
//@ run-rustfix

use std::arch::global_asm;

fn main() {}

global_asm!("", options(nomem, readonly, noreturn, raw));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

