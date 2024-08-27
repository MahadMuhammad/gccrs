#![feature(specialization)]
#![allow(incomplete_features)]

trait Trait {
    type Type;
}

impl Trait for i32 {
    default type Type = i32;
}

struct Wrapper<const C: <i32 as Trait>::Type> {}
// { dg-error "" "" { target *-*-* } .-1 }

impl<const C: usize> Wrapper<C> {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

fn main() {}

