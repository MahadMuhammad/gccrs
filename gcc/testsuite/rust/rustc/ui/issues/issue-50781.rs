trait Trait {}

trait X {
    fn foo(&self) where Self: Trait;
}

impl X for () {
    fn foo(&self) {}
}

impl Trait for dyn X {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }

pub fn main() {
    // Check that this does not segfault.
    <dyn X as X>::foo(&());
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
}

