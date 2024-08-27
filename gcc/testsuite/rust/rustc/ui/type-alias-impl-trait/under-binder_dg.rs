#![feature(type_alias_impl_trait)]

type Opaque<'a> = impl Sized + 'a;

fn test(f: fn(u8)) -> fn(Opaque<'_>) {
    f // { dg-error ".E0792." "" { target *-*-* } }
}

fn main() {}

