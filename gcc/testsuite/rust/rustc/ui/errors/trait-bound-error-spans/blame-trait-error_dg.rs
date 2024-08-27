trait T1 {}
trait T2 {}
trait T3 {}
trait T4 {}

impl<B: T2> T1 for Wrapper<B> {}

impl T2 for i32 {}
impl T3 for i32 {}

impl<A: T3> T2 for Burrito<A> {}

struct Wrapper<W> {
    value: W,
}

struct Burrito<F> {
    filling: F,
}

impl<It: Iterator> T1 for Option<It> {}

impl<'a, A: T1> T1 for &'a A {}

fn want<V: T1>(_x: V) {}

enum ExampleTuple<T> {
    ExampleTupleVariant(T),
}
use ExampleDifferentTupleVariantName as ExampleYetAnotherTupleVariantName;
use ExampleTuple as ExampleOtherTuple;
use ExampleTuple::ExampleTupleVariant as ExampleDifferentTupleVariantName;
use ExampleTuple::*;

impl<A> T1 for ExampleTuple<A> where A: T3 {}

enum ExampleStruct<T> {
    ExampleStructVariant { field: T },
}
use ExampleDifferentStructVariantName as ExampleYetAnotherStructVariantName;
use ExampleStruct as ExampleOtherStruct;
use ExampleStruct::ExampleStructVariant as ExampleDifferentStructVariantName;
use ExampleStruct::*;

impl<A> T1 for ExampleStruct<A> where A: T3 {}

struct ExampleActuallyTupleStruct<T>(T, i32);
use ExampleActuallyTupleStruct as ExampleActuallyTupleStructOther;

impl<A> T1 for ExampleActuallyTupleStruct<A> where A: T3 {}

fn example<Q>(q: Q) {
    want(Wrapper { value: Burrito { filling: q } });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(Some(()));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(Some(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&Some(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleTuple::ExampleTupleVariant(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleTupleVariant(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleOtherTuple::ExampleTupleVariant(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleDifferentTupleVariantName(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleYetAnotherTupleVariantName(q));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleStruct::ExampleStructVariant { field: q });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleStructVariant { field: q });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleOtherStruct::ExampleStructVariant { field: q });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleDifferentStructVariantName { field: q });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleYetAnotherStructVariantName { field: q });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleActuallyTupleStruct(q, 0));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    want(&ExampleActuallyTupleStructOther(q, 0));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

