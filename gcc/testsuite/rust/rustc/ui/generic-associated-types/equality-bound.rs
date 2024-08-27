fn sum<I: Iterator<Item = ()>>(i: I) -> i32 where I::Item = i32 {
// { dg-error "" "" { target *-*-* } .-1 }
    panic!()
}
fn sum2<I: Iterator>(i: I) -> i32 where I::Item = i32 {
// { dg-error "" "" { target *-*-* } .-1 }
    panic!()
}
fn sum3<J: Iterator>(i: J) -> i32 where I::Item = i32 {
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }
    panic!()
}

use std::iter::FromIterator;

struct X {}

impl FromIterator<bool> for X {
    fn from_iter<T>(_: T) -> Self where T: IntoIterator, IntoIterator::Item = A,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
    {
        todo!()
    }
}

struct Y {}

impl FromIterator<bool> for Y {
    fn from_iter<T>(_: T) -> Self where T: IntoIterator, T::Item = A,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
    {
        todo!()
    }
}

struct Z {}

impl FromIterator<bool> for Z {
    fn from_iter<T: IntoIterator>(_: T) -> Self where IntoIterator::Item = A,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
    {
        todo!()
    }
}

struct K {}

impl FromIterator<bool> for K {
    fn from_iter<T: IntoIterator>(_: T) -> Self where T::Item = A,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
    {
        todo!()
    }
}

struct L {}

impl FromIterator<bool> for L {
    fn from_iter<T>(_: T) -> Self where IntoIterator::Item = A, T: IntoIterator,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
    {
        todo!()
    }
}

struct M {}

impl FromIterator<bool> for M {
    fn from_iter<T>(_: T) -> Self where T::Item = A, T: IntoIterator,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
    {
        todo!()
    }
}
fn main() {}

