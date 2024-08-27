//@ compile-flags: -Znext-solver

#![feature(negative_bounds, negative_impls)]

trait Trait {}
impl !Trait for () {}

fn produce() -> impl !Trait {}
fn consume(_: impl Trait) {}

fn main() {
    consume(produce()); // { dg-error ".E0277." "" { target *-*-* } }
}

fn weird0() -> impl Sized + !Sized {}
// { dg-error ".E0271." "" { target *-*-* } .-1 }
fn weird1() -> impl !Sized + Sized {}
// { dg-error ".E0271." "" { target *-*-* } .-1 }
fn weird2() -> impl !Sized {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

