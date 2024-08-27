//@ ignore-test (auxiliary, used by other tests)

#[derive(Print)]
enum ProceduralMasqueradeDummyType {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
// { dg-error "" "" { target *-*-* } .-7 }
// { dg-warning "" "" { target *-*-* } .-8 }
    Input
}

