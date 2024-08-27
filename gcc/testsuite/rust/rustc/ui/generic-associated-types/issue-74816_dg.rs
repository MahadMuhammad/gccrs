//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(associated_type_defaults)]

trait Trait1 {
    fn foo();
}

trait Trait2 {
    type Associated: Trait1 = Self;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

impl Trait2 for () {}

fn call_foo<T: Trait2>() {
    T::Associated::foo()
}

fn main() {
    call_foo::<()>()
}

