//@ run-rustfix

#![feature(generic_const_items)]
#![allow(incomplete_features, dead_code)]

const K<T>: u64
where
    T: Tr<()>
= T::K;
// { dg-error "" "" { target *-*-* } .-3 }

trait Tr<P> {
    const K: u64
    where
        P: Copy
    = 0;
// { dg-error "" "" { target *-*-* } .-3 }
}

fn main() {}

