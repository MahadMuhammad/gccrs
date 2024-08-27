//@ run-rustfix
#![allow(dead_code)]
trait Foo {
    type Bar;
}

fn foo<T: Foo>()
where
    T::Baa: std::fmt::Debug,
// { dg-error ".E0220." "" { target *-*-* } .-1 }
{
}

fn bar<T>()
where
    T::Baa: std::fmt::Debug,
// { dg-error ".E0220." "" { target *-*-* } .-1 }
{
}

fn main() {}

