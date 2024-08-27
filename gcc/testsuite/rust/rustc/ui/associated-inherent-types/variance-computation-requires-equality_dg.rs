//@ check-pass

#![feature(inherent_associated_types)]
// { dg-warning "" "" { target *-*-* } .-1 }

struct D<T> {
  a: T
}

impl<T: Default> D<T> {
    type Item = T;

    fn next() -> Self::Item {
        Self::Item::default()
    }
}


fn main() {
}

