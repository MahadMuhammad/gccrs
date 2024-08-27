pub struct Bar<S>(S);

pub trait Foo {}

impl<S> Foo for Bar<S> where for<'a> <&'a S>::Item: Foo {}
// { dg-error ".E0223." "" { target *-*-* } .-1 }

fn main() {}

