//@ needs-asm-support
#![feature(naked_functions)]

use std::arch::asm;

#[track_caller] // { dg-error ".E0737." "" { target *-*-* } }
// { dg-error ".E0737." "" { target *-*-* } .-1 }
#[naked]
extern "C" fn f() {
    unsafe {
        asm!("", options(noreturn));
    }
}

struct S;

impl S {
    #[track_caller] // { dg-error ".E0737." "" { target *-*-* } }
// { dg-error ".E0737." "" { target *-*-* } .-1 }
    #[naked]
    extern "C" fn g() {
        unsafe {
            asm!("", options(noreturn));
        }
    }
}

fn main() {}

