//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(auto_traits)]
#![feature(negative_impls)]

unsafe auto trait Trait {
    type Output; // { dg-error "" "" { target *-*-* } }
}

fn call_method<T: Trait>(x: T) {}

fn main() {
    // ICE
    call_method(());
}

