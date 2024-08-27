//@ revisions: edition2021 edition2024
// { dg-additional-options "-frust-edition=2021" }
// { dg-additional-options "-frust-edition=2024" }
//@[edition2024] compile-flags: -Zunstable-options

unsafe extern "C" {
    unsafe static TEST1: i32;
    unsafe fn test1(i: i32);
}

fn test2() {
    unsafe {
        test1(TEST1);
    }
}

fn test3() {
    test1(TEST1);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

