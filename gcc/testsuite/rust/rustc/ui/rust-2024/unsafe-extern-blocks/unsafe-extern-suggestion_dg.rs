//@ run-rustfix

#![deny(missing_unsafe_on_extern)]
#![allow(unused)]

extern "C" {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    static TEST1: i32;
    fn test1(i: i32);
}

unsafe extern "C" {
    static TEST2: i32;
    fn test2(i: i32);
}

fn main() {}

