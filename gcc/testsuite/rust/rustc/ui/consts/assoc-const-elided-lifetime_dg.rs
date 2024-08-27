#![deny(elided_lifetimes_in_associated_constant)]

use std::marker::PhantomData;

struct Foo<'a> {
    x: PhantomData<&'a ()>,
}

impl<'a> Foo<'a> {
    const FOO: Foo<'_> = Foo { x: PhantomData::<&()> };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    const BAR: &() = &();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

fn main() {}

