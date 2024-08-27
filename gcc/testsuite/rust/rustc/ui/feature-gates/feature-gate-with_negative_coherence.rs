trait Foo { }

impl<T: std::ops::DerefMut> Foo for T { }

impl<T> Foo for &T { }
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() { }

