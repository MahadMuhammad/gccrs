#![feature(closure_lifetime_binder)]
fn main() {
    for<> |_: &'a ()| -> () {};
// { dg-error ".E0261." "" { target *-*-* } .-1 }
    for<'a> |_: &'b ()| -> () {};
// { dg-error ".E0261." "" { target *-*-* } .-1 }
}

