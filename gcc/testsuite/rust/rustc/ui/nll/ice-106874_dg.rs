// issue: rust-lang/rust#106874
// ICE BoundUniversalRegionError

use std::marker::PhantomData;
use std::rc::Rc;

pub fn func<V, F: Fn(&mut V)>(f: F) -> A<impl X> {
    A(B(C::new(D::new(move |st| f(st)))))
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-error "" "" { target *-*-* } .-6 }
// { dg-error "" "" { target *-*-* } .-7 }
// { dg-error "" "" { target *-*-* } .-8 }
// { dg-error "" "" { target *-*-* } .-9 }
// { dg-error "" "" { target *-*-* } .-10 }
}

trait X {}
trait Y {
    type V;
}

struct A<T>(T);

struct B<T>(Rc<T>);
impl<T> X for B<T> {}

struct C<T: Y>(T::V);
impl<T: Y> C<T> {
    fn new(_: T) -> Rc<Self> {
        todo!()
    }
}
struct D<V, F>(F, PhantomData<fn(&mut V)>);

impl<V, F> D<V, F> {
    fn new(_: F) -> Self {
        todo!()
    }
}
impl<V, F: Fn(&mut V)> Y for D<V, F> {
    type V = V;
}

pub fn main() {}

