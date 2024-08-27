#![feature(type_alias_impl_trait)]

type Tait = impl Sized;

struct One;
fn one() -> Tait { One }

struct Two<T>(T);
fn two() -> Tait { Two::<()>(todo!()) }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

