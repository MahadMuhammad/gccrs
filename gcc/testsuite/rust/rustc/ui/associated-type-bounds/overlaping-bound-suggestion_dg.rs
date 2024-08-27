#![allow(bare_trait_objects)]
trait Item {
    type Core;
}
pub struct Flatten<I> {
    inner: <IntoIterator<Item: IntoIterator<Item: >>::IntoIterator as Item>::Core,
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { dg-error ".E0223." "" { target *-*-* } .-2 }
}

fn main() {}

