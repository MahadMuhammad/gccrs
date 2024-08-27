//@ needs-llvm-components: avr
//@ compile-flags: --target=avr-unknown-gnu-atmega328 --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang="sized"]
trait Sized { }

// Test that the AVR interrupt ABI cannot be used when avr_interrupt
// feature gate is not used.

extern "avr-non-blocking-interrupt" fn fu() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "avr-interrupt" fn f() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

trait T {
    extern "avr-interrupt" fn m();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "avr-non-blocking-interrupt" fn mu();
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    extern "avr-interrupt" fn dm() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "avr-non-blocking-interrupt" fn dmu() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

struct S;
impl T for S {
    extern "avr-interrupt" fn m() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "avr-non-blocking-interrupt" fn mu() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

impl S {
    extern "avr-interrupt" fn im() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "avr-non-blocking-interrupt" fn imu() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

type TA = extern "avr-interrupt" fn();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
type TAU = extern "avr-non-blocking-interrupt" fn();
// { dg-error ".E0658." "" { target *-*-* } .-1 }

extern "avr-interrupt" {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "avr-non-blocking-interrupt" {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

