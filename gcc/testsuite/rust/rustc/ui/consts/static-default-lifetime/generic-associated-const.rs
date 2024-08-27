#![deny(elided_lifetimes_in_associated_constant)]
#![feature(generic_const_items)]
// { dg-warning "" "" { target *-*-* } .-1 }

struct A;
impl A {
    const GAC_TYPE<T>: &str = "";
    const GAC_LIFETIME<'a>: &str = "";
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

trait Trait {
    const GAC_TYPE<T>: &str = "";
    const GAC_LIFETIME<'a>: &str = "";
// { dg-error ".E0106." "" { target *-*-* } .-1 }
}

fn main() {}

