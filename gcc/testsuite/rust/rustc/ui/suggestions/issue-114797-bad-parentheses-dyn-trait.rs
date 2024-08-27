//@run-rustfix
#![allow(dead_code)]

trait Trait {}

fn assert_send() -> *mut dyn (Trait + Send) {
// { dg-error "" "" { target *-*-* } .-1 }
    loop {}
}

fn foo2(_: &dyn (Trait + Send)) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn foo3(_: &dyn(Trait + Send)) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

