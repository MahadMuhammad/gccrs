#[link(kind = "link-arg", name = "foo")]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "C" {}

fn main() {}

