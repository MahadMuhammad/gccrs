// { dg-additional-options "-frust-edition=2021" }

pub trait Trait<'a, T> {}

pub struct Struct<T>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }
pub enum Enum<T> {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }

pub union Union<T> {
// { dg-error ".E0392." "" { target *-*-* } .-1 }
    f1: usize,
}

impl<'a, T> Struct<T> for Trait<'a, T> {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }

impl<'a, T> Enum<T> for Trait<'a, T> {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }

impl<'a, T> Union<T> for Trait<'a, T> {}
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }

fn main() {}

