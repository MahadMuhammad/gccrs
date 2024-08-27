//@ compile-flags: -Zdeduplicate-diagnostics=yes

const N: usize = 8;
#[repr(align(N))]
// { dg-error ".E0693." "" { target *-*-* } .-1 }
struct T;

#[repr(align('a'))]
// { dg-error ".E0589." "" { target *-*-* } .-1 }
struct H;

#[repr(align("str"))]
// { dg-error ".E0589." "" { target *-*-* } .-1 }
struct L;

#[repr(align())]
// { dg-error ".E0693." "" { target *-*-* } .-1 }
struct X;

const P: usize = 8;
#[repr(packed(P))]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
struct A;

#[repr(packed())]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
struct B;

#[repr(packed)]
struct C;

fn main() {}

