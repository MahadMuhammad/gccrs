trait Foo<T, T = T> {}
// { dg-error ".E0403." "" { target *-*-* } .-1 }

fn eq<A, B>() {
    eq::<dyn, Foo>
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { dg-warning ".E0107." "" { target *-*-* } .-3 }
// { dg-warning ".E0107." "" { target *-*-* } .-4 }
}

fn main() {}

