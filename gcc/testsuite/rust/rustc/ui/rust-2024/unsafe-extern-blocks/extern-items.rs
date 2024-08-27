//@ revisions: edition2021 edition2024
// { dg-additional-options "-frust-edition=2021" }
//@[edition2021] check-pass
// { dg-additional-options "-frust-edition=2024" }
//@[edition2024] compile-flags: -Zunstable-options

extern "C" {
// { dg-error "" "" { target *-*-* } .-1 }
    static TEST1: i32;
    fn test1(i: i32);
}

unsafe extern "C" {
    static TEST2: i32;
    fn test2(i: i32);
}

fn main() {}

