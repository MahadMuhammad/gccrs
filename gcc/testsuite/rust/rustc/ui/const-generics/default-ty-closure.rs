// https://github.com/rust-lang/rust/issues/116796

struct X<const FN: fn() = { || {} }>;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

