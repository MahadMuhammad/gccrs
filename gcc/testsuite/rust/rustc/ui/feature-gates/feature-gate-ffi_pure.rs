#![crate_type = "lib"]

extern "C" {
    #[ffi_pure] // { dg-error ".E0658." "" { target *-*-* } }
    pub fn foo();
}

