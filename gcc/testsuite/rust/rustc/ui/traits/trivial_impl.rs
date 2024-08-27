//! This test checks that we do need to implement
//! all members, even if their where bounds only hold
//! due to other impls.

trait Foo<T> {
    fn foo()
    where
        Self: Foo<()>;
}

impl Foo<()> for () {
    fn foo() {}
}

impl Foo<u32> for () {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

fn main() {}

