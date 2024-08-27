#![feature(type_alias_impl_trait)]

type Opaque<'a> = impl Sized + 'a;

trait Trait<'a> {
    type Assoc;
}

impl<'a> Trait<'a> for () {
    type Assoc = ();
}

fn test() -> &'static dyn for<'a> Trait<'a, Assoc = Opaque<'a>> {
    &()
// { dg-error ".E0792." "" { target *-*-* } .-1 }
}

fn main() {}

