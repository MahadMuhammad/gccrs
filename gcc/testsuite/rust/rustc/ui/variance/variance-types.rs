#![allow(dead_code)]
#![feature(rustc_attrs)]

use std::cell::Cell;

// Check that a type parameter which is only used in a trait bound is
// not considered bivariant.

#[rustc_variance]
struct InvariantMut<'a,A:'a,B:'a> { // { dg-error "" "" { target *-*-* } }
    t: &'a mut (A,B)
}

#[rustc_variance]
struct InvariantCell<A> { // { dg-error "" "" { target *-*-* } }
    t: Cell<A>
}

#[rustc_variance]
struct InvariantIndirect<A> { // { dg-error "" "" { target *-*-* } }
    t: InvariantCell<A>
}

#[rustc_variance]
struct Covariant<A> { // { dg-error "" "" { target *-*-* } }
    t: A, u: fn() -> A
}

#[rustc_variance]
struct Contravariant<A> { // { dg-error "" "" { target *-*-* } }
    t: fn(A)
}

#[rustc_variance]
enum Enum<A,B,C> { // { dg-error "" "" { target *-*-* } }
    Foo(Covariant<A>),
    Bar(Contravariant<B>),
    Zed(Covariant<C>,Contravariant<C>)
}

pub fn main() { }

