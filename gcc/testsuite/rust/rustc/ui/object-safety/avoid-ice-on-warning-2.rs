//@ revisions: old new
// { dg-additional-options "-frust-edition=2015" }
// { dg-additional-options "-frust-edition=2021" }
fn id<F>(f: Copy) -> usize {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
    f()
// { dg-error "" "" { target *-*-* } .-1 }
}
fn main() {}

