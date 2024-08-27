fn foo<T: impl Trait>() {}
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {}

