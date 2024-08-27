// Test that const fn is illegal in a trait declaration, whether or
// not a default is provided, and even with the feature gate.

trait Foo {
    const fn f() -> u32;
// { dg-error ".E0379." "" { target *-*-* } .-1 }
    const fn g() -> u32 {
// { dg-error ".E0379." "" { target *-*-* } .-1 }
        0
    }
}

fn main() {}

