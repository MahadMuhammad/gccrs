struct Foo {}

impl<const UNUSED: usize> Drop for Foo {}
// { dg-error ".E0207." "" { target *-*-* } .-1 }
// { dg-error ".E0207." "" { target *-*-* } .-2 }

fn main() {}

