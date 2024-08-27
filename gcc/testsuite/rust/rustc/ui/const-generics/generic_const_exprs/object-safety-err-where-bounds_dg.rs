#![feature(generic_const_exprs)]
#![allow(incomplete_features)]


const fn bar<T: ?Sized>() -> usize { 7 }

trait Foo {
    fn test(&self) where [u8; bar::<Self>()]: Sized;
}

impl Foo for () {
    fn test(&self) where [u8; bar::<Self>()]: Sized {}
}

fn use_dyn(v: &dyn Foo) {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
    v.test();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

fn main() {}

