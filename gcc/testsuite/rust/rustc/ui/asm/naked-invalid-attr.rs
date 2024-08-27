// Checks that #[naked] attribute can be placed on function definitions only.
//
//@ needs-asm-support
#![feature(naked_functions)]
#![naked] // { dg-error "" "" { target *-*-* } }

use std::arch::asm;

extern "C" {
    #[naked] // { dg-error "" "" { target *-*-* } }
    fn f();
}

#[naked] // { dg-error "" "" { target *-*-* } }
#[repr(C)]
struct S {
    a: u32,
    b: u32,
}

trait Invoke {
    #[naked] // { dg-error "" "" { target *-*-* } }
    extern "C" fn invoke(&self);
}

impl Invoke for S {
    #[naked]
    extern "C" fn invoke(&self) {
        unsafe { asm!("", options(noreturn)) }
    }
}

#[naked]
extern "C" fn ok() {
    unsafe { asm!("", options(noreturn)) }
}

impl S {
    #[naked]
    extern "C" fn g() {
        unsafe { asm!("", options(noreturn)) }
    }

    #[naked]
    extern "C" fn h(&self) {
        unsafe { asm!("", options(noreturn)) }
    }
}

fn main() {
    #[naked] || {}; // { dg-error "" "" { target *-*-* } }
}

