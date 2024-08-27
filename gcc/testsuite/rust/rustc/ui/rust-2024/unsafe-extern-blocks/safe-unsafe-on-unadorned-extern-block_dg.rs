//@ revisions: edition2021 edition2024
// { dg-additional-options "-frust-edition=2021" }
// { dg-additional-options "-frust-edition=2024" }
//@[edition2024] compile-flags: -Zunstable-options

extern "C" {
// { dg-error "" "" { target *-*-* } .-1 }
    safe static TEST1: i32;
// { dg-error "" "" { target *-*-* } .-1 }
    safe fn test1(i: i32);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test2() {
    test1(TEST1);
}

fn main() {}

