//@ needs-asm-support

use std::arch::asm;

#[naked]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "C" fn naked() {
    asm!("", options(noreturn))
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

#[naked]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "C" fn naked_2() -> isize {
    asm!("", options(noreturn))
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

fn main() {}

