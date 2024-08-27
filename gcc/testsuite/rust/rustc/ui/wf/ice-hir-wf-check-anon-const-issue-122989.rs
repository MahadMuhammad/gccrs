// Regression test for ICE #122989
trait Foo<const N: Bar<2>> {
// { dg-warning ".E0038." "" { target *-*-* } .-1 }
// { dg-warning ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }
    fn func() {}
}

trait Bar<const M: Foo<2>> {}
// { dg-warning ".E0038." "" { target *-*-* } .-1 }
// { dg-warning ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }

fn main() {}

