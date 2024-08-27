#![feature(associated_const_equality)]

trait T {
    type A: S<C<X = 0i32> = 34>;
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
}

trait S {
    const C: i32;
}

fn main() {}

