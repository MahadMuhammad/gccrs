//@ build-pass
//@ needs-unwind

#![warn(ffi_unwind_calls)]

mod foo {
    #[no_mangle]
    pub extern "C-unwind" fn foo() {}
}

extern "C-unwind" {
    fn foo();
}

fn main() {
    // Call to Rust function is fine.
    foo::foo();
    // Call to foreign function should warn.
    unsafe { foo(); }
// { dg-warning "" "" { target *-*-* } .-1 }
    let ptr: extern "C-unwind" fn() = foo::foo;
    // Call to function pointer should also warn.
    ptr();
// { dg-warning "" "" { target *-*-* } .-1 }
}

