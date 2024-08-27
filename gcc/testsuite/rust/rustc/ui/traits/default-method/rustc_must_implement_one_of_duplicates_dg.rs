#![feature(rustc_attrs)]

#[rustc_must_implement_one_of(a, a)]
// { dg-error "" "" { target *-*-* } .-1 }
trait Trait {
    fn a() {}
}

#[rustc_must_implement_one_of(b, a, a, c, b, c)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
trait Trait1 {
    fn a() {}
    fn b() {}
    fn c() {}
}

fn main() {}

