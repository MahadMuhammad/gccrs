#![allow(non_fmt_panics)]
#![crate_type = "lib"]

#[track_caller]
const fn a() -> u32 {
    panic!("hey")
}

#[track_caller]
const fn b() -> u32 {
    a()
}

const fn c() -> u32 {
    b()
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-note ".E0080." "" { target *-*-* } .-2 }
// { dg-note ".E0080." "" { target *-*-* } .-3 }
}

const X: u32 = c();
// { dg-note "" "" { target *-*-* } .-1 }

