// See https://github.com/rust-lang/rust/issues/88475
//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
//@ check-pass
#![warn(array_into_iter)]
#![allow(unused)]

struct FooIter;

trait MyIntoIter {
    fn into_iter(self) -> FooIter;
}

impl<T, const N: usize> MyIntoIter for [T; N] {
    fn into_iter(self) -> FooIter {
        FooIter
    }
}

struct Point;

pub fn main() {
    let points: [Point; 1] = [Point];
    let y = points.into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

