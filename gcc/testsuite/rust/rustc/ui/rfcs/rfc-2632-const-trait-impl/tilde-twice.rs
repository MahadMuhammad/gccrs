//@ compile-flags: -Z parse-only

#![feature(const_trait_impl)]

struct S<T: ~const ~const Tr>;
// { dg-error "" "" { target *-*-* } .-1 }

