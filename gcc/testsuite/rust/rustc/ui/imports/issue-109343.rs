#![crate_type = "lib"]

pub mod f {}
pub use unresolved::f;
// { dg-error ".E0432." "" { target *-*-* } .-1 }

/// [g]
pub use f as g;

fn main() {}

