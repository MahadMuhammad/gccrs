fn f(_: extern "Rust" fn()) {}
extern "C" fn bar() {}

fn main() { f(bar) }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

