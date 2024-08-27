#![deny(elided_lifetimes_in_associated_constant)]

trait Bar<'a> {
    const STATIC: &'a str;
}

struct A;
impl Bar<'_> for A {
    const STATIC: &str = "";
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-warning ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}

struct B;
impl Bar<'static> for B {
    const STATIC: &str = "";
}

fn main() {}

