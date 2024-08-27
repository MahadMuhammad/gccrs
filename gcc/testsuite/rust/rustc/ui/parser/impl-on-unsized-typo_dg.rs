trait Tr {}

impl<T ?Sized> Tr for T {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

