// { dg-additional-options "-frust-edition=2021" }

use std::iter;

fn f<T>(data: &[T]) -> impl Iterator<Item = Vec> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
    iter::empty()
}

fn g<T>(data: &[T], target: T) -> impl Iterator<Item = Vec<T>> {
    f(data).filter(|x| x == target)
}

fn main() {}

