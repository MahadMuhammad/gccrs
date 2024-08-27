//! This test checks that we currently need to implement
//! members, even if their where bounds don't hold for the impl type.

trait Foo {
    fn foo()
    where
        Self: Sized;
}

impl Foo for () {
    fn foo() {}
}

// Must not be allowed
impl Foo for i32 {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

// Should be allowed
impl Foo for dyn std::fmt::Debug {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

impl Foo for dyn std::fmt::Display {
    fn foo() {}
}

fn main() {}

