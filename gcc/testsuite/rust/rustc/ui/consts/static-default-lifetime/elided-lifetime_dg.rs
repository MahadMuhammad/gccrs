#![deny(elided_lifetimes_in_associated_constant)]

struct Foo<'a>(&'a ());

impl Foo<'_> {
    const STATIC: &str = "";
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

trait Bar {
    const STATIC: &str;
}

impl Bar for Foo<'_> {
    const STATIC: &str = "";
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-warning ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}

fn main() {}

