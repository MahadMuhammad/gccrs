//@ run-rustfix
//@ aux-build:external_unsafe_macro.rs

#![deny(unsafe_op_in_unsafe_fn)] // { dg-note "" "" { target *-*-* } }
#![crate_name = "wrapping_unsafe_block_sugg"]

extern crate external_unsafe_macro;

unsafe fn unsf() {}

pub unsafe fn foo() {
// { dg-note "" "" { target *-*-* } .-1 }
    unsf(); // { dg-error ".E0133." "" { target *-*-* } }
// { dg-note ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
    unsf(); // { dg-error ".E0133." "" { target *-*-* } }
// { dg-note ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
}

pub unsafe fn bar(x: *const i32) -> i32 {
// { dg-note "" "" { target *-*-* } .-1 }
    let y = *x; // { dg-error ".E0133." "" { target *-*-* } }
// { dg-note ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
    y + *x // { dg-error ".E0133." "" { target *-*-* } }
// { dg-note ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
}

static mut BAZ: i32 = 0;
pub unsafe fn baz() -> i32 {
// { dg-note "" "" { target *-*-* } .-1 }
    let y = BAZ; // { dg-error ".E0133." "" { target *-*-* } }
// { dg-note ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
    y + BAZ // { dg-error ".E0133." "" { target *-*-* } }
// { dg-note ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
}

macro_rules! unsafe_macro { () => (unsf()) }
// { dg-error ".E0133." "" { target *-*-* } .-1 }
// { dg-note ".E0133." "" { target *-*-* } .-2 }
// { dg-note ".E0133." "" { target *-*-* } .-3 }
// { dg-note ".E0133." "" { target *-*-* } .-4 }
// { dg-error ".E0133." "" { target *-*-* } .-5 }
// { dg-note ".E0133." "" { target *-*-* } .-6 }
// { dg-note ".E0133." "" { target *-*-* } .-7 }
// { dg-note ".E0133." "" { target *-*-* } .-8 }

pub unsafe fn unsafe_in_macro() {
// { dg-note "" "" { target *-*-* } .-1 }
    unsafe_macro!();
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    unsafe_macro!();
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
}

pub unsafe fn unsafe_in_external_macro() {
    // FIXME: https://github.com/rust-lang/rust/issues/112504
    // FIXME: ~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    external_unsafe_macro::unsafe_macro!();
    external_unsafe_macro::unsafe_macro!();
}

fn main() {}

