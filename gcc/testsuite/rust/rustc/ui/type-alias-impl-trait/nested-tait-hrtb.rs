#![feature(type_alias_impl_trait)]

trait Trait<'a> { type Assoc; }
impl<'a> Trait<'a> for () { type Assoc = &'a str; }

type WithoutLt = impl Sized;
fn without_lt() -> impl for<'a> Trait<'a, Assoc = WithoutLt> {}
// { dg-error ".E0700." "" { target *-*-* } .-1 }

type WithLt<'a> = impl Sized + 'a;

fn with_lt() -> impl for<'a> Trait<'a, Assoc = WithLt<'a>> {}
// { dg-error ".E0792." "" { target *-*-* } .-1 }

fn main() {}

