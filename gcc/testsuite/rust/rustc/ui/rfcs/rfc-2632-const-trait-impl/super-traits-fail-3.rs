//@ compile-flags: -Znext-solver
#![allow(incomplete_features)]
#![feature(const_trait_impl, effects)]

//@ revisions: yy yn ny nn
//@[yy] known-bug: #110395

#[cfg_attr(any(yy, yn), const_trait)]
trait Foo {
    fn a(&self);
}

#[cfg_attr(any(yy, ny), const_trait)]
trait Bar: ~const Foo {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

const fn foo<T: ~const Bar>(x: &T) {
// { dg-error "" "" { target *-*-* } .-1 }
    x.a();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

