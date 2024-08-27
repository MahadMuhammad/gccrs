#![feature(type_alias_impl_trait)]

type Opaque2<T> = impl Sized;
type Opaque<'a, T> = Opaque2<T>;
fn defining<'a, T>(x: &'a i32) -> Opaque<T> { x }
// { dg-error ".E0700." "" { target *-*-* } .-1 }

fn main() {}

