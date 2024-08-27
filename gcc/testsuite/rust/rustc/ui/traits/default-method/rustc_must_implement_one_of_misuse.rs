#![feature(rustc_attrs)]

#[rustc_must_implement_one_of(a, b)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
trait Tr0 {}

#[rustc_must_implement_one_of(a, b)]
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr1 {
    fn a() {}
}

#[rustc_must_implement_one_of(a)]
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr2 {
    fn a() {}
}

#[rustc_must_implement_one_of]
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr3 {}

#[rustc_must_implement_one_of(A, B)]
trait Tr4 {
    const A: u8 = 1; // { dg-error "" "" { target *-*-* } }

    type B; // { dg-error "" "" { target *-*-* } }
}

#[rustc_must_implement_one_of(a, b)]
trait Tr5 {
    fn a(); // { dg-error "" "" { target *-*-* } }

    fn b(); // { dg-error "" "" { target *-*-* } }
}

#[rustc_must_implement_one_of(abc, xyz)]
// { dg-error "" "" { target *-*-* } .-1 }
fn function() {}

#[rustc_must_implement_one_of(abc, xyz)]
// { dg-error "" "" { target *-*-* } .-1 }
struct Struct {}

fn main() {}

