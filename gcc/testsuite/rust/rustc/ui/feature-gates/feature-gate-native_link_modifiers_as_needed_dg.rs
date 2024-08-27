#[link(name = "foo", kind = "dylib", modifiers = "+as-needed")]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "C" {}

fn main() {}

