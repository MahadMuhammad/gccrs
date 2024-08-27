#![feature(closure_lifetime_binder)]

fn main() {
    for<T> || -> () {};
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

