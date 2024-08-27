// Regression test for #81712.

trait A {
    type BType: B<AType = Self>;
}

trait B {
    type AType: A<BType = Self>;
}
trait C {
    type DType<T>: D<T, CType = Self>;
}
trait D<T> {
    type CType: C<DType = Self>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
}

fn main() {}

