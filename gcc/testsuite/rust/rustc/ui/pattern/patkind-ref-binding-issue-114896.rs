//@ run-rustfix
#![allow(dead_code)]

fn main() {
    fn x(a: &char) {
        let &b = a;
        b.make_ascii_uppercase();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
    }
}

