// Regression test for the ICE described in #86721.

//@ revisions: rev1 rev2
#![cfg_attr(any(), rev1, rev2)]
#![crate_type = "lib"]

#[cfg(any(rev1))]
trait T {
    const U: usize = return;
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(any(rev2))]
trait T2 {
    fn foo(a: [(); return]);
// { dg-error "" "" { target *-*-* } .-1 }
}

