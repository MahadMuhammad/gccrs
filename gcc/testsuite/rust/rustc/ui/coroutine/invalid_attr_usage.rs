//! The `coroutine` attribute is only allowed on closures.

#![feature(coroutines)]

#[coroutine]
// { dg-error "" "" { target *-*-* } .-1 }
struct Foo;

#[coroutine]
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {}

