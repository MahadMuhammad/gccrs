//! This test checks that we currently need to implement
//! members, even if their where bounds don't hold for the impl type.

trait Foo<T> {
    fn foo()
    where
        Self: Foo<()>;
}

impl Foo<u32> for () {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

fn main() {}

