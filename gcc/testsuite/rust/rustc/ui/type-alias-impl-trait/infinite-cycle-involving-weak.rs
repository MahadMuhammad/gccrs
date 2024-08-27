#![feature(type_alias_impl_trait)]

type T = impl Copy;
// { dg-error ".E0720." "" { target *-*-* } .-1 }

static STATIC: T = None::<&'static T>;

fn main() {}

