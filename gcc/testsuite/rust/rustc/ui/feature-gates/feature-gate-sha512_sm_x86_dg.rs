//@ only-x86_64
#[target_feature(enable = "sha512")]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
unsafe fn foo() {}

fn main() {}

