#[cfg(relocation_model = "pic")] // { dg-error ".E0658." "" { target *-*-* } }
fn _foo() {}

fn main() {}

