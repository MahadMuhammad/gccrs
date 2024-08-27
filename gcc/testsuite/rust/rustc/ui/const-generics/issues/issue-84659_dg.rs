//@ run-rustfix
#![allow(incomplete_features, dead_code, unused_braces)]
#![feature(generic_const_exprs)]

trait Bar<const N: usize> {}

trait Foo<'a> {
    const N: usize;
    type Baz: Bar<{ Self::N }>;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

