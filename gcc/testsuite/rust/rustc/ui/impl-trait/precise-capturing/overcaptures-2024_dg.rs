//@ run-rustfix

#![allow(unused)]
#![deny(impl_trait_overcaptures)]

fn named<'a>(x: &'a i32) -> impl Sized { *x }
// { dg-error "" "" { target *-*-* } .-1 }

fn implicit(x: &i32) -> impl Sized { *x }
// { dg-error "" "" { target *-*-* } .-1 }

struct W;
impl W {
    fn hello(&self, x: &i32) -> impl Sized + '_ { self }
// { dg-error "" "" { target *-*-* } .-1 }
}

trait Higher<'a> {
    type Output;
}
impl Higher<'_> for () {
    type Output = ();
}

fn hrtb() -> impl for<'a> Higher<'a, Output = impl Sized> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

