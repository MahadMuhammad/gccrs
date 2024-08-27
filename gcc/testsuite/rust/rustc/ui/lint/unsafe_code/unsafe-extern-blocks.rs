#![deny(unsafe_code)]

#[allow(unsafe_code)]
unsafe extern "C" {
    fn foo();
}

unsafe extern "C" {
// { dg-error "" "" { target *-*-* } .-1 }
    fn bar();
}

fn main() {}

