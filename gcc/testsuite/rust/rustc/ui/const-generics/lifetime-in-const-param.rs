// https://github.com/rust-lang/rust/issues/113462

struct S2<'b>(&'b ());

struct S<'a, const N: S2>(&'a ());
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

fn main() {}

