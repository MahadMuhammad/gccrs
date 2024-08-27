//@ check-fail
// Fixes #119830

#![feature(effects)] // { dg-warning "" "" { target *-*-* } }
#![feature(min_specialization)]
#![feature(const_trait_impl)]

trait Specialize {}

trait Foo {}

impl<T> const Foo for T {}
// { dg-error "" "" { target *-*-* } .-1 }

impl<T> const Foo for T where T: const Specialize {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

fn main() {}

