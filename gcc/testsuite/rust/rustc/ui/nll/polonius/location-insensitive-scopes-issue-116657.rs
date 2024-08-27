// This is a non-regression test for issue #116657, where NLL and `-Zpolonius=next` computed
// different loan scopes when a member constraint was not ultimately applied.

//@ revisions: nll polonius
//@ [polonius] compile-flags: -Zpolonius=next

#![feature(impl_trait_in_assoc_type)]

trait Callable {
    type Output;
    fn call(x: Self) -> Self::Output;
}

trait PlusOne {}

impl<'a> PlusOne for &'a mut i32 {}

impl<T: PlusOne> Callable for T {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    type Output = impl PlusOne;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn test<'a>(y: &'a mut i32) -> impl PlusOne {
    <&mut i32 as Callable>::call(y)
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

