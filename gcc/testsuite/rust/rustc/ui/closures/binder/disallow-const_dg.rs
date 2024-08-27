#![feature(closure_lifetime_binder)]

fn main() {
    for<const N: i32> || -> () {};
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

