#![feature(rustc_attrs)]

#[rustc_legacy_const_generics(0)] // { dg-error "" "" { target *-*-* } }
fn foo1() {}

#[rustc_legacy_const_generics(1)] // { dg-error "" "" { target *-*-* } }
fn foo2<const X: usize>() {}

#[rustc_legacy_const_generics(2)] // { dg-error "" "" { target *-*-* } }
fn foo3<const X: usize>(_: u8) {}

#[rustc_legacy_const_generics(a)] // { dg-error "" "" { target *-*-* } }
fn foo4<const X: usize>() {}

#[rustc_legacy_const_generics(1, a, 2, b)] // { dg-error "" "" { target *-*-* } }
fn foo5<const X: usize, const Y: usize, const Z: usize, const W: usize>() {}

#[rustc_legacy_const_generics(0)] // { dg-error "" "" { target *-*-* } }
struct S;

#[rustc_legacy_const_generics(0usize)] // { dg-error "" "" { target *-*-* } }
fn foo6<const X: usize>() {}

extern {
    #[rustc_legacy_const_generics(1)] // { dg-error "" "" { target *-*-* } }
    fn foo7<const X: usize>(); // { dg-error ".E0044." "" { target *-*-* } }
}

#[rustc_legacy_const_generics(0)] // { dg-error "" "" { target *-*-* } }
fn foo8<X>() {}

impl S {
    #[rustc_legacy_const_generics(0)] // { dg-error "" "" { target *-*-* } }
    fn foo9<const X: usize>() {}
}

#[rustc_legacy_const_generics] // { dg-error "" "" { target *-*-* } }
fn bar1() {}

#[rustc_legacy_const_generics = 1] // { dg-error "" "" { target *-*-* } }
fn bar2() {}

fn main() {}

