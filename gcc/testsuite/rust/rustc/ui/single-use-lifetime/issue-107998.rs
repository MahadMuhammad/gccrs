#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'a>(x: &'a i32) -> R) -> R {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    f(&3)
}

fn main() {}

