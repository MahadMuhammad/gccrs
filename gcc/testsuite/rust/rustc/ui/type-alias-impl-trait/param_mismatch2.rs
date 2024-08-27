//! This test checks that when checking for opaque types that
//! only differ in lifetimes, we handle the case of non-generic
//! regions correctly.
#![feature(type_alias_impl_trait)]

fn id(s: &str) -> &str {
    s
}

type Opaque<'a> = impl Sized + 'a;

fn test(s: &str) -> (impl Fn(&str) -> Opaque<'_>, impl Fn(&str) -> Opaque<'_>) {
    (id, id) // { dg-error ".E0792." "" { target *-*-* } }
}

fn main() {}

