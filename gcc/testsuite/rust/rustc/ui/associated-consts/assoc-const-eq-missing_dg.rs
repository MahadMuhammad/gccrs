#![feature(associated_const_equality)]
#![allow(unused)]

pub trait Foo {
  const N: usize;
}

pub struct Bar;

impl Foo for Bar {
  const N: usize = 3;
}

fn foo1<F: Foo<Z = 3>>() {}
// { dg-error ".E0220." "" { target *-*-* } .-1 }
fn foo2<F: Foo<Z = usize>>() {}
// { dg-error ".E0220." "" { target *-*-* } .-1 }
fn foo3<F: Foo<Z = 5>>() {}
// { dg-error ".E0220." "" { target *-*-* } .-1 }

fn main() {
  foo1::<Bar>();
  foo2::<Bar>();
  foo3::<Bar>();
}

