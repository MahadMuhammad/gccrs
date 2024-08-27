//@ revisions: old next
//@[next] compile-flags: -Znext-solver=coherence
#![feature(coerce_unsized)]
#![feature(unsize)]

use std::marker::Unsize;
use std::ops::CoerceUnsized;

struct Wrapper<T>(T);

impl<'a, 'b, T, S> CoerceUnsized<&'a Wrapper<T>> for &'b Wrapper<S> where S: Unsize<T> {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}

