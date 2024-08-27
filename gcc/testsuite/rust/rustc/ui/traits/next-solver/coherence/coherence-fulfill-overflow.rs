//@ compile-flags: -Znext-solver=coherence

#![recursion_limit = "10"]

trait Trait {}

struct W<T: ?Sized>(*const T);
trait TwoW {}
impl<T: ?Sized + TwoW> TwoW for W<W<T>> {}

impl<T: ?Sized + TwoW> Trait for W<T> {}
impl<T: ?Sized + TwoW> Trait for T {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

