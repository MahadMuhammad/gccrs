#![feature(intrinsics)]

trait Foo {
    extern "rust-intrinsic" fn foo(&self); // { dg-error "" "" { target *-*-* } }
}

impl Foo for () {
    extern "rust-intrinsic" fn foo(&self) { // { dg-error "" "" { target *-*-* } }
    }
}

extern "rust-intrinsic" fn hello() {// { dg-error ".E0093." "" { target *-*-* } }
// { dg-error ".E0093." "" { target *-*-* } .-1 }
}

fn main() {
}

