#![crate_type = "lib"]

extern "C" {
    #[ffi_const] // { dg-error ".E0658." "" { target *-*-* } }
    pub fn foo();
}

