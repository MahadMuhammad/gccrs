// See https://github.com/rust-lang/rust/issues/88475
//@ run-rustfix
// { dg-additional-options "-frust-edition=2021" }
//@ check-pass
#![warn(boxed_slice_into_iter)]
#![allow(unused)]

struct FooIter;

trait MyIntoIter {
    fn into_iter(self) -> FooIter;
}

impl<T> MyIntoIter for Box<[T]> {
    fn into_iter(self) -> FooIter {
        FooIter
    }
}

struct Point;

pub fn main() {
    let points: Box<[_]> = vec![Point].into_boxed_slice();
    let y = points.into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

