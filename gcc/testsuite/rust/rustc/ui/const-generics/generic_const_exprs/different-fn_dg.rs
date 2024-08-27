#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::mem::size_of;
use std::marker::PhantomData;

struct Foo<T>(PhantomData<T>);

fn test<T>() -> [u8; size_of::<T>()] {
    [0; size_of::<Foo<T>>()]
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

fn main() {
    test::<u32>();
}

