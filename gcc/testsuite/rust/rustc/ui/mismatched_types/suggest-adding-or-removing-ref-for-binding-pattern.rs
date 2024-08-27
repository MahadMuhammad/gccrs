//@ run-rustfix
#![allow(dead_code, unused_variables)]

fn main() {
    enum Blah {
        A(isize, isize, usize),
        B(isize, usize),
    }

    match Blah::A(1, 1, 2) {
        Blah::A(_, x, ref y) | Blah::B(x, y) => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }

    match Blah::A(1, 1, 2) {
        Blah::A(_, x, y) | Blah::B(x, ref y) => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

