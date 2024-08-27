//@ only-x86_64
//@ needs-asm-support

// Test to make sure that we emit const errors eagerly for inline asm

use std::arch::asm;

fn test<T>() {
    unsafe {
        asm!("/* {} */", const 1 / 0);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    }
}

fn main() {}

