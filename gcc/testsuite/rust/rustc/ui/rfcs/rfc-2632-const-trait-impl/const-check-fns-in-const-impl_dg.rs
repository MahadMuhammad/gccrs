#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

struct S;
#[const_trait]
trait T {
    fn foo();
}

fn non_const() {}

impl const T for S {
    fn foo() { non_const() }
// { dg-error ".E0015." "" { target *-*-* } .-1 }
}

fn main() {}

