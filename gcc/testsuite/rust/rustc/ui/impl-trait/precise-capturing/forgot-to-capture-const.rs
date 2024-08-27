fn constant<const C: usize>() -> impl Sized + use<> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

