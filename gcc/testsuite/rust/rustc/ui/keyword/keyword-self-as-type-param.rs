// Regression test of #36638.

struct Foo<Self>(Self);
// { dg-error ".E0072." "" { target *-*-* } .-1 }
// { dg-error ".E0072." "" { target *-*-* } .-2 }

trait Bar<Self> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

