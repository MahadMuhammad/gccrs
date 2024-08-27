// gate-test-intrinsics
//@ compile-flags: --crate-type=rlib

#![feature(no_core, lang_items)]
#![no_core]

#[lang="sized"]
trait Sized { }

#[lang="tuple_trait"]
trait Tuple { }

// Functions
extern "rust-intrinsic" fn f1() {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "rust-intrinsic" fn f2() {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "rust-call" fn f4(_: ()) {} // { dg-error ".E0658." "" { target *-*-* } }

// Methods in trait definition
trait Tr {
    extern "rust-intrinsic" fn m1(); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "rust-intrinsic" fn m2(); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "rust-call" fn m4(_: ()); // { dg-error ".E0658." "" { target *-*-* } }

    extern "rust-call" fn dm4(_: ()) {} // { dg-error ".E0658." "" { target *-*-* } }
}

struct S;

// Methods in trait impl
impl Tr for S {
    extern "rust-intrinsic" fn m1() {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "rust-intrinsic" fn m2() {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "rust-call" fn m4(_: ()) {} // { dg-error ".E0658." "" { target *-*-* } }
}

// Methods in inherent impl
impl S {
    extern "rust-intrinsic" fn im1() {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "rust-intrinsic" fn im2() {} // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    extern "rust-call" fn im4(_: ()) {} // { dg-error ".E0658." "" { target *-*-* } }
}

// Function pointer types
type A1 = extern "rust-intrinsic" fn(); // { dg-error ".E0658." "" { target *-*-* } }
type A2 = extern "rust-intrinsic" fn(); // { dg-error ".E0658." "" { target *-*-* } }
type A4 = extern "rust-call" fn(_: ()); // { dg-error ".E0658." "" { target *-*-* } }

// Foreign modules
extern "rust-intrinsic" {} // { dg-error ".E0658." "" { target *-*-* } }
extern "rust-intrinsic" {} // { dg-error ".E0658." "" { target *-*-* } }
extern "rust-call" {} // { dg-error ".E0658." "" { target *-*-* } }

