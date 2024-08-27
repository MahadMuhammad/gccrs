#[rustc_doc_primitive = "usize"]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
/// Some docs
mod usize {}

fn main() {}

