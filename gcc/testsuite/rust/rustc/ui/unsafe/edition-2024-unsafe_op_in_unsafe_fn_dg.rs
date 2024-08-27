// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Zunstable-options
//@ check-pass
#![crate_type = "lib"]
#![deny(unused_unsafe)]

unsafe fn unsf() {}

unsafe fn foo() {
    unsf();
// { dg-warning "" "" { target *-*-* } .-1 }

    // no unused_unsafe
    unsafe {
        unsf();
    }
}

