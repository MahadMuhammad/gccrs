//@ run-rustfix
#![deny(unused_parens)]

fn main() {
    if let(Some(_))= Some(1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    for(_x)in 1..10 {}
// { dg-error "" "" { target *-*-* } .-1 }

    if(2 == 1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    // reported by parser
    for(_x in 1..10) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

