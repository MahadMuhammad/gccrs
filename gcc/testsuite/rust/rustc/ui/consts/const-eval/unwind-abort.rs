#![feature(const_extern_fn)]

const extern "C" fn foo() {
    panic!() // { dg-error ".E0080." "" { target *-*-* } }
}

const _: () = foo();
// Ensure that the CTFE engine handles calls to `extern "C"` aborting gracefully

fn main() {
    foo();
}

