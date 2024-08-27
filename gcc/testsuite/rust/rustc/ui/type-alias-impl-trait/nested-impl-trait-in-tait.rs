#![feature(type_alias_impl_trait)]

pub type Tait = impl Iterator<Item = (&'db LocalKey, impl Iterator)>;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
// { dg-error ".E0412." "" { target *-*-* } .-3 }
// { dg-error ".E0412." "" { target *-*-* } .-4 }

pub fn main() {}

