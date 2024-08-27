#![deny(rust_2024_compatibility)]
#![crate_type = "lib"]

unsafe fn unsf() {}

unsafe fn foo() {
    unsf();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

