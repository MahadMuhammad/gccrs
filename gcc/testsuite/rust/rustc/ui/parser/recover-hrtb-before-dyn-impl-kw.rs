trait Trait {}

fn test(_: &for<'a> dyn Trait) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn test2(_: for<'a> impl Trait) {}
// { dg-error "" "" { target *-*-* } .-1 }

// Issue #118564
type A2 = dyn<for<> dyn>;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

