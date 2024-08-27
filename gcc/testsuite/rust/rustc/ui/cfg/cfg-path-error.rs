//@ check-fail

#![allow(unexpected_cfgs)] // invalid cfgs

#[cfg(any(foo, foo::bar))]
// { dg-error "" "" { target *-*-* } .-1 }
fn foo1() {}

#[cfg(any(foo::bar, foo))]
// { dg-error "" "" { target *-*-* } .-1 }
fn foo2() {}

#[cfg(all(foo, foo::bar))]
// { dg-error "" "" { target *-*-* } .-1 }
fn foo3() {}

#[cfg(all(foo::bar, foo))]
// { dg-error "" "" { target *-*-* } .-1 }
fn foo4() {}

fn main() {}

