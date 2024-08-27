#![feature(anonymous_lifetime_in_impl_trait)]

// The same thing should happen for constraints in dyn trait.
fn f(x: &mut dyn Iterator<Item: Iterator<Item = &'_ ()>>) -> Option<&'_ ()> { x.next() }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

fn main() {}

