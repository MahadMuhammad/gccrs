#![allow(incomplete_features)]
#![feature(explicit_tail_calls)]

pub const fn f() {
    become g();
}

const fn g() {
    panic!()
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-note ".E0080." "" { target *-*-* } .-2 }
// { dg-note ".E0080." "" { target *-*-* } .-3 }
// { dg-note ".E0080." "" { target *-*-* } .-4 }
}

const _: () = f();
// { dg-note "" "" { target *-*-* } .-1 }

fn main() {}

