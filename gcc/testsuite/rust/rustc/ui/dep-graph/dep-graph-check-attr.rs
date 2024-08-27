// Test that using rustc_clean/dirty/if_this_changed/then_this_would_need
// are forbidden when `-Z query-dep-graph` is not enabled.

#![feature(rustc_attrs)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[rustc_clean(hir_owner)] // { dg-error "" "" { target *-*-* } }
fn main() {}

#[rustc_if_this_changed(hir_owner)] // { dg-error "" "" { target *-*-* } }
struct Foo<T> {
    f: T,
}

#[rustc_clean(hir_owner)] // { dg-error "" "" { target *-*-* } }
type TypeAlias<T> = Foo<T>;

#[rustc_then_this_would_need(variances_of)] // { dg-error "" "" { target *-*-* } }
trait Use<T> {}

