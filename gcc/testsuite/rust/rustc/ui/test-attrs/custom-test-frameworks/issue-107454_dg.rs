//@ compile-flags: --test

#![feature(custom_test_frameworks)]
#![deny(unnameable_test_items)]

fn foo() {
    #[test_case]
// { dg-error "" "" { target *-*-* } .-1 }
    fn test2() {}
}

