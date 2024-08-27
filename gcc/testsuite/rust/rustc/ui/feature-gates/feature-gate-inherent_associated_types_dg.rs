// Test that inherent associated types cannot be used when inherent_associated_types
// feature gate is not used.

struct Foo;

impl Foo {
    type Bar = isize; // { dg-error ".E0658." "" { target *-*-* } }
}

fn main() {}

