#![allow(incomplete_features)]
#![feature(explicit_tail_calls)]

const fn f() {
    if false {
        become not_const();
// { dg-error ".E0015." "" { target *-*-* } .-1 }
    }
}

const fn g((): ()) {
    if false {
        become yes_const(not_const());
// { dg-error ".E0015." "" { target *-*-* } .-1 }
    }
}

fn not_const() {}

const fn yes_const((): ()) {}

fn main() {}

