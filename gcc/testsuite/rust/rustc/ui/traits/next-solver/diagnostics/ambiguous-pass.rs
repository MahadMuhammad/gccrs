//@ compile-flags: -Znext-solver

trait Trait {
    type Assoc;
}

struct W<T>(*mut T);
impl<T> Trait for W<W<T>>
where
    W<T>: Trait,
{
    type Assoc = ();
}

trait NoOverlap {}
impl<T: Trait> NoOverlap for T {}

impl<T: Trait<Assoc = ()>> NoOverlap for W<T> {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

