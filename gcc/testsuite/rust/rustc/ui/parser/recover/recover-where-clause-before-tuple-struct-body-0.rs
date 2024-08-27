// Regression test for issues #100790 and #106439.
//@ run-rustfix

pub struct Example
where
    (): Sized,
(#[allow(dead_code)] usize);
// { dg-error "" "" { target *-*-* } .-3 }

struct _Demo
where
    (): Sized,
    String: Clone,
(pub usize, usize);
// { dg-error "" "" { target *-*-* } .-4 }

fn main() {}

