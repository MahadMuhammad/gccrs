use std::ops::DerefMut;

trait Foo {}
impl<T: DerefMut> Foo for T {}
impl<U> Foo for &U {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

