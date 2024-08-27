struct A;
struct B;
struct C;
struct D;

fn f(
    a1: A,
    a2: A,
    b1: B,
    b2: B,
    c1: C,
    c2: C,
) {}

fn main() {
    f(C, A, A, A, B, B, C); // { dg-error ".E0061." "" { target *-*-* } }
    f(C, C, A, A, B, B);  // { dg-error ".E0308." "" { target *-*-* } }
    f(A, A, D, D, B, B);  // { dg-error ".E0308." "" { target *-*-* } }
    f(C, C, B, B, A, A);  // { dg-error ".E0308." "" { target *-*-* } }
    f(C, C, A, B, A, A);  // { dg-error ".E0308." "" { target *-*-* } }
}

