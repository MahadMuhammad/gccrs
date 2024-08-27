//@ aux-build:legacy-const-generics.rs

extern crate legacy_const_generics;

fn foo<const N: usize>() {
    let a = 1;
    legacy_const_generics::foo(0, a, 2);
// { dg-error ".E0435." "" { target *-*-* } .-1 }

    legacy_const_generics::foo(0, N, 2);

    legacy_const_generics::foo(0, N + 1, 2);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

