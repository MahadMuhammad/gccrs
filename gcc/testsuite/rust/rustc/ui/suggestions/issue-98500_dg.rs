//@ aux-build:not-object-safe.rs

extern crate not_object_safe;

pub trait B where
    Self: not_object_safe::A,
{
    fn f2(&self);
}

struct S(Box<dyn B>);
// { dg-error ".E0038." "" { target *-*-* } .-1 }

fn main() {}

