#![feature(unsize)]

use std::marker::Unsize;
use std::ops::Deref;

trait Foo: Bar {}
trait Bar {}

impl<T> Bar for T where dyn Foo: Unsize<dyn Bar> {}
impl Bar for () {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

