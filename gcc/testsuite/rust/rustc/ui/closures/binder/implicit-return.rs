#![feature(closure_lifetime_binder)]

fn main() {
    let _f = for<'a> |_: &'a ()| {};
// { dg-error "" "" { target *-*-* } .-1 }
}

