//@ revisions: good bad

//@[good] known-bug: unknown
// `for<T> T: 'static` doesn't imply itself when processing outlives obligations

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn foo() where for<T> T: 'static {}

#[cfg(bad)]
fn bad() {
    foo();
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(good)]
fn good() where for<T> T: 'static {
    foo();
}

fn main() {}

