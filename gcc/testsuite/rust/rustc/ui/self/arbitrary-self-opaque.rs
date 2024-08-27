#![feature(type_alias_impl_trait)]
struct Foo;

type Bar = impl Sized;
// { dg-error "" "" { target *-*-* } .-1 }

impl Foo {
    fn foo(self: Bar) {}
// { dg-error ".E0307." "" { target *-*-* } .-1 }
// { dg-error ".E0307." "" { target *-*-* } .-2 }
}

fn main() {}

