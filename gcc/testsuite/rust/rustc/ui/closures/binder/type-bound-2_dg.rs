#![feature(closure_lifetime_binder, non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    for<T> || -> () {};
// { dg-error "" "" { target *-*-* } .-1 }
}

