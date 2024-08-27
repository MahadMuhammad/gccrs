#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use std::marker::PhantomData;

trait SadBee {
    const ASSOC: usize;
}
// fn(&'static ())` is a supertype of `for<'a> fn(&'a ())` while
// we allow two different impls for these types, leading
// to different const eval results.
impl SadBee for for<'a> fn(&'a ()) {
    const ASSOC: usize = 0;
}
impl SadBee for fn(&'static ()) {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    const ASSOC: usize = 100;
}

struct Foo<T: SadBee>([u8; <T as SadBee>::ASSOC], PhantomData<T>)
where
    [(); <T as SadBee>::ASSOC]:;

fn covariant(v: &'static Foo<for<'a> fn(&'a ())>) -> &'static Foo<fn(&'static ())> {
    v
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {
    let y = covariant(&Foo([], PhantomData));
    println!("{:?}", y.0);
}

