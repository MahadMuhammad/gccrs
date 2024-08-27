use std::borrow::Borrow;
use std::cmp::Ordering;
use std::marker::PhantomData;

#[derive(PartialEq, Default)]
// { dg-error ".E0119." "" { target *-*-* } .-1 }
pub(crate) struct Interval<T>(PhantomData<T>);

// This impl overlaps with the `derive` unless we reject the nested
// `Interval<?1>: PartialOrd<Interval<?1>>` candidate which results
// in a -- currently inductive -- cycle.
impl<T, Q> PartialEq<Q> for Interval<T>
where
    T: Borrow<Q>,
    Q: ?Sized + PartialOrd,
{
    fn eq(&self, _: &Q) -> bool {
        true
    }
}

impl<T, Q> PartialOrd<Q> for Interval<T>
where
    T: Borrow<Q>,
    Q: ?Sized + PartialOrd,
{
    fn partial_cmp(&self, _: &Q) -> Option<Ordering> {
        None
    }
}

fn main() {}

