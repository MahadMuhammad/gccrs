//@ check-pass
//@ run-rustfix
//@ edition 2018
#![warn(rust_2021_prelude_collisions)]

trait MyTrait<A> {
    fn from_iter(x: Option<A>);
}

impl<T> MyTrait<()> for Vec<T> {
    fn from_iter(_: Option<()>) {}
}

fn main() {
    <Vec<i32>>::from_iter(None);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

