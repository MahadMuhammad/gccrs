//@ compile-flags: -Znext-solver

#![feature(lazy_type_alias)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Foo {}

type A<T: Foo> = T;

struct W<T>(T);

// For `W<A<usize>>` to be WF, `A<usize>: Sized` must hold. However, when assembling
// alias bounds for `A<usize>`, we try to normalize it, but it doesn't hold because
// `usize: Foo` doesn't hold. Therefore we ICE, because we don't expect to still
// encounter weak types in `assemble_alias_bound_candidates_recur`.
fn hello(_: W<A<usize>>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}

