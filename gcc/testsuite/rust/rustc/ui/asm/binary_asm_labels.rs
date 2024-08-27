//@ needs-asm-support
//@ only-x86_64

// tests that labels containing only the digits 0 and 1 are rejected
// uses of such labels can sometimes be interpreted as a binary literal

use std::arch::{asm, global_asm};

fn main() {
    unsafe {
        asm!("0: jmp 0b"); // { dg-error "" "" { target *-*-* } }
        asm!("1: jmp 1b"); // { dg-error "" "" { target *-*-* } }
        asm!("10: jmp 10b"); // { dg-error "" "" { target *-*-* } }
        asm!("01: jmp 01b"); // { dg-error "" "" { target *-*-* } }
        asm!("1001101: jmp 1001101b"); // { dg-error "" "" { target *-*-* } }
    }
}

