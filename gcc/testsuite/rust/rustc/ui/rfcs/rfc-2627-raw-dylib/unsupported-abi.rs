//@ only-x86_64
//@ only-windows
//@ compile-flags: --crate-type lib --emit link
#[link(name = "foo", kind = "raw-dylib")]
extern "stdcall" {
    fn f(x: i32);
// { dg-error "" "" { target *-*-* } .-1 }
}

pub fn lib_main() {
    unsafe { f(42); }
}

