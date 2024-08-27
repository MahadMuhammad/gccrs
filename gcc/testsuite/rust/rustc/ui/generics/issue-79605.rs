struct X<'a, T>(&'a T);

impl X<'_, _> {}
// { dg-error ".E0121." "" { target *-*-* } .-1 }

fn main() {}

