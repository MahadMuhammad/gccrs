#![allow(warnings)]

trait Trait<T> {
    fn foo(_: T) {}
}

pub struct Foo<T = Box<Trait<DefaultFoo>>>;  // { dg-error ".E0392." "" { target *-*-* } }
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }
type DefaultFoo = Foo;

fn main() {
}

