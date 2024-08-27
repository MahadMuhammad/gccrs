#![feature(ffi_pure)]
#![crate_type = "lib"]

#[ffi_pure] // { dg-error ".E0755." "" { target *-*-* } }
pub fn foo() {}

#[ffi_pure] // { dg-error ".E0755." "" { target *-*-* } }
macro_rules! bar {
    () => ()
}

extern "C" {
    #[ffi_pure] // { dg-error ".E0755." "" { target *-*-* } }
    static INT: i32;
}

