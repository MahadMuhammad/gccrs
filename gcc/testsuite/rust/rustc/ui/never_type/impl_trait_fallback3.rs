#![feature(type_alias_impl_trait)]

fn main() {}

trait T {
    type Assoc;
}

type Foo = impl T;

fn a() -> Foo {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    // This is not a defining use, it doesn't actually constrain the opaque type.
    panic!()
}

