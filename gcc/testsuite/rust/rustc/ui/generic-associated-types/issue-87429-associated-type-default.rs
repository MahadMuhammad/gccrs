//@ check-fail

#![feature(associated_type_defaults)]

trait Family {
    // Fine, i32: PartialEq<i32>
    type Member<'a>: for<'b> PartialEq<Self::Member<'b>> = i32;
}

struct Foo;
trait Family2 {
    // Not fine, not Foo: PartialEq<Foo>
    type Member<'a>: for<'b> PartialEq<Self::Member<'b>> = Foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

