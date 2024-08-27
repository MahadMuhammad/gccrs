#![feature(type_alias_impl_trait)]

type Foo = impl Sized;

fn bar() -> Foo {
    None
// { dg-error ".E0282." "" { target *-*-* } .-1 }
}

fn baz() -> Foo {
    Some(())
}

fn main() {}

