#![feature(do_not_recommend)]

pub trait Foo {}

impl Foo for i32 {}

pub trait Bar {}

#[diagnostic::do_not_recommend]
impl<T: Foo> Bar for T {}

fn stuff<T: Bar>(_: T) {}

fn main() {
    stuff(1u8);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

