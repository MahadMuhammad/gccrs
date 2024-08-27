//@ only-x86_64

use std::arch::asm;

fn main() {
    unsafe {
        asm!("jmp {}", label {});
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    }
}

