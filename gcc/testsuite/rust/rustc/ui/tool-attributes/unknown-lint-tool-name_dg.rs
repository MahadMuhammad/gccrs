#![deny(foo::bar)] // { dg-error ".E0710." "" { target *-*-* } }
// { dg-error ".E0710." "" { target *-*-* } .-2 }

#[allow(foo::bar)] // { dg-error ".E0710." "" { target *-*-* } }
// { dg-error ".E0710." "" { target *-*-* } .-2 }
fn main() {}

