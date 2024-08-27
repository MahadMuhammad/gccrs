//@ only-x86_64
#[target_feature(enable = "xop")]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
unsafe fn foo() {}

fn main() {}

