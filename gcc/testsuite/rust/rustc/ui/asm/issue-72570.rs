//@ needs-asm-support

use std::arch::asm;

fn main() {
    unsafe {
        asm!("", in("invalid") "".len());
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

