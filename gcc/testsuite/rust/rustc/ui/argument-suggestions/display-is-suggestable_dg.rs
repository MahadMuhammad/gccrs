use std::fmt::Display;

fn foo(x: &(dyn Display + Send)) {}

fn main() {
    foo();
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

