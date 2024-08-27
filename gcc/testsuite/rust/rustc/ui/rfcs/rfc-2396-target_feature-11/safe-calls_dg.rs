//@ only-x86_64
// Set the base cpu explicitly, in case the default has been changed.
//@ compile-flags: -C target-cpu=x86-64

#![feature(target_feature_11)]

#[target_feature(enable = "sse2")]
const fn sse2() {}

#[target_feature(enable = "sse2")]
#[target_feature(enable = "fxsr")]
const fn sse2_and_fxsr() {}

#[target_feature(enable = "avx")]
#[target_feature(enable = "bmi2")]
fn avx_bmi2() {}

struct Quux;

impl Quux {
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "bmi2")]
    fn avx_bmi2(&self) {}
}

fn foo() {
    sse2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
    avx_bmi2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
    Quux.avx_bmi2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

#[target_feature(enable = "sse2")]
fn bar() {
    sse2();
    avx_bmi2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
    Quux.avx_bmi2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

#[target_feature(enable = "avx")]
fn baz() {
    sse2();
    avx_bmi2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
    Quux.avx_bmi2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

#[target_feature(enable = "avx")]
#[target_feature(enable = "bmi2")]
fn qux() {
    sse2();
    avx_bmi2();
    Quux.avx_bmi2();
}

const _: () = sse2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }

const _: () = sse2_and_fxsr();
// { dg-error ".E0133." "" { target *-*-* } .-1 }

#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn needs_unsafe_block() {
    sse2();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

fn main() {}

