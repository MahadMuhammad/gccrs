struct A;
struct B;

fn f(b1: B, b2: B, a2: C) {} // { dg-error ".E0412." "" { target *-*-* } }

fn main() {
    f(A, A, B, C); // { dg-error ".E0061." "" { target *-*-* } }
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

