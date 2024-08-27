fn hello(_: impl Sized + use<>) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

