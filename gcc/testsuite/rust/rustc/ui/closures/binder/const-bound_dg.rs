#![feature(closure_lifetime_binder, non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn main()  {
    for<const N: i32> || -> () {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

