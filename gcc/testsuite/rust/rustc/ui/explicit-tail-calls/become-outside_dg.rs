//@ revisions: constant array
#![allow(incomplete_features)]
#![feature(explicit_tail_calls)]

#[cfg(constant)]
const _: () = {
    become f(); // { dg-error "" "" { target *-*-* } }
};

#[cfg(array)]
struct Bad([(); become f()]); // { dg-error "" "" { target *-*-* } }

fn f() {}

fn main() {}

