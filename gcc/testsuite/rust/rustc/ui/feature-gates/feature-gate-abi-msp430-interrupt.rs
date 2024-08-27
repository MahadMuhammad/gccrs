//@ needs-llvm-components: msp430
//@ compile-flags: --target=msp430-none-elf --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang="sized"]
trait Sized { }

extern "msp430-interrupt" fn f() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

trait T {
    extern "msp430-interrupt" fn m();
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    extern "msp430-interrupt" fn dm() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

struct S;
impl T for S {
    extern "msp430-interrupt" fn m() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

impl S {
    extern "msp430-interrupt" fn im() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

type TA = extern "msp430-interrupt" fn();
// { dg-error ".E0658." "" { target *-*-* } .-1 }

extern "msp430-interrupt" {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

