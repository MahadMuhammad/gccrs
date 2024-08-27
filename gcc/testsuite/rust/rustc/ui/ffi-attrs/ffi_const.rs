#![feature(ffi_const)]
#![crate_type = "lib"]

#[ffi_const] // { dg-error ".E0756." "" { target *-*-* } }
pub fn foo() {}

#[ffi_const] // { dg-error ".E0756." "" { target *-*-* } }
macro_rules! bar {
    () => ()
}

extern "C" {
    #[ffi_const] // { dg-error ".E0756." "" { target *-*-* } }
    static INT: i32;
}

