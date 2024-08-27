//@ revisions: stock gated
// gate-test-const_trait_impl

#![cfg_attr(gated, feature(const_trait_impl))]
#![feature(rustc_attrs)]

struct S;
#[const_trait] // { dg-error "" "" { target *-*-* } }
trait T {}
impl const T for S {}
// { dg-error "" "" { target *-*-* } .-1 }

const fn f<A: ~const T>() {} // { dg-error "" "" { target *-*-* } }
fn g<A: const T>() {} // { dg-error "" "" { target *-*-* } }

macro_rules! discard { ($ty:ty) => {} }

discard! { impl ~const T } // { dg-error "" "" { target *-*-* } }
discard! { impl const T } // { dg-error "" "" { target *-*-* } }

#[rustc_error]
fn main() {} // { dg-error "" "" { target *-*-* } }

