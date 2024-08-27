#![feature(type_alias_impl_trait)]

type Foo = impl PartialEq<(Foo, i32)>;

struct Bar;

impl PartialEq<(Bar, i32)> for Bar {
    fn eq(&self, _other: &(Bar, i32)) -> bool {
        true
    }
}

fn foo() -> Foo {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    Bar
}

fn main() {}

