#![feature(type_alias_impl_trait)]

type X = impl Sized;

trait Foo {
    type Bar: Iterator<Item = X>;
}

impl Foo for () {
    type Bar = std::vec::IntoIter<u32>;
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

fn incoherent() -> X {
    22_i32
}

fn main() {}

