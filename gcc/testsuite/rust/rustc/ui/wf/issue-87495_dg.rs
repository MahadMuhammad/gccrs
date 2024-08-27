// Regression test for the ICE described in #87495.

trait T {
    const CONST: (bool, dyn T);
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

fn main() {}

