//@ build-fail
//@ needs-asm-support

use std::arch::global_asm;

fn main() {}

global_asm!("/* {} */", const 1 << 500); // { dg-error ".E0080." "" { target *-*-* } }

global_asm!("/* {} */", const 1 / 0); // { dg-error ".E0080." "" { target *-*-* } }

