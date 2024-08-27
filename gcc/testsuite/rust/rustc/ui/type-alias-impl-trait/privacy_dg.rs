//@ check-pass

#![feature(type_alias_impl_trait)]

type Foo = (impl Sized, u8);
pub fn foo() -> Foo {
// { dg-warning "" "" { target *-*-* } .-1 }
    (42, 42)
}
fn main() {}

