#![feature(ffi_const, ffi_pure)]

extern "C" {
    #[ffi_pure] // { dg-error ".E0757." "" { target *-*-* } }
    #[ffi_const]
    pub fn baz();
}

fn main() {
    unsafe { baz() };
}

