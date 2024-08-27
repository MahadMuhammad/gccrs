//@ revisions: e2021 e2024

// { dg-additional-options "-frust-edition=2021" }
//@ [e2024] compile-flags: --edition 2024 -Z unstable-options

fn main() {
    static mut X: i32 = 1;

    let _x = &X;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

