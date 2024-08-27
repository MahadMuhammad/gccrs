// repr currently doesn't support literals
#[repr("C")] // { dg-error ".E0565." "" { target *-*-* } }
struct A {}

fn main() {}

