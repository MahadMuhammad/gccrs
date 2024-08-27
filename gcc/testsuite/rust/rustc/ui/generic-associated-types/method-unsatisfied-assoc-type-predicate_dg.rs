// Test that the predicate printed in an unresolved method error prints the
// generics for a generic associated type.

trait X {
    type Y<T>;
}

trait M { // { dg-note "" "" { target *-*-* } }
    fn f(&self) {}
}

impl<T: X<Y<i32> = i32>> M for T {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

struct S;
// { dg-note "" "" { target *-*-* } .-1 }

impl X for S {
    type Y<T> = bool;
}

fn f(a: S) {
    a.f();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }
}

fn main() {}

