//@ compile-flags: -Znext-solver

// Test that we don't incorrectly leak unconstrained inference variables
// if the projection contained an error. This caused an ICE in writeback.

trait Mirror {
    type Assoc: ?Sized;
}

struct Wrapper<T: ?Sized>(T);
impl<T: ?Sized> Mirror for Wrapper<T> {
    type Assoc = T;
}

fn mirror<W: Mirror>(_: W) -> Box<W::Assoc> { todo!() }

fn type_error() -> TypeError { todo!() }
// { dg-error ".E0412." "" { target *-*-* } .-1 }

fn main() {
    let x = mirror(type_error());
}

