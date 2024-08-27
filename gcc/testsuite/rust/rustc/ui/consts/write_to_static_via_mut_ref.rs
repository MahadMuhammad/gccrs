#![feature(const_mut_refs)]

static OH_NO: &mut i32 = &mut 42; // { dg-error ".E0764." "" { target *-*-* } }
fn main() {
    assert_eq!(*OH_NO, 42);
    *OH_NO = 43; // { dg-error ".E0594." "" { target *-*-* } }
}

