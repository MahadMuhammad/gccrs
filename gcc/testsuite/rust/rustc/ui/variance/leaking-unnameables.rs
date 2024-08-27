// Test variance computation doesn't explode when we leak unnameable
// types due to `-> _` recovery.

pub struct Type<'a>(&'a ());

pub fn g() {}

pub fn f<T>() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
   g
}

fn main() {}

