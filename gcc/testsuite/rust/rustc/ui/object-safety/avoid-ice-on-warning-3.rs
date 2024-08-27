//@ revisions: old new
// { dg-additional-options "-frust-edition=2015" }
// { dg-additional-options "-frust-edition=2021" }
trait B { fn f(a: A) -> A; }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
// { dg-warning "" "" { target *-*-* } .-7 }
trait A { fn g(b: B) -> B; }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
// { dg-warning "" "" { target *-*-* } .-7 }
fn main() {}

