#[derive_const(Default)] // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
pub struct S;

fn main() {}

