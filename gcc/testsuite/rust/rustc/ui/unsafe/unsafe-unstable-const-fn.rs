#![stable(feature = "foo", since = "1.33.0")]
#![feature(staged_api)]

#[stable(feature = "foo", since = "1.33.0")]
#[rustc_const_unstable(feature = "const_foo", issue = "none")]
const fn unstable(a: *const i32, b: i32) -> bool {
    *a == b
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

fn main() {}

