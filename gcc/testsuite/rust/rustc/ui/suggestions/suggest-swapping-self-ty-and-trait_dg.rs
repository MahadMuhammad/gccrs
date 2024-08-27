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
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { dg-warning ".E0404." "" { target *-*-* } .-2 }
// { dg-warning ".E0404." "" { target *-*-* } .-3 }

impl<'a, T> Enum<T> for Trait<'a, T> {}
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { dg-warning ".E0404." "" { target *-*-* } .-2 }
// { dg-warning ".E0404." "" { target *-*-* } .-3 }

impl<'a, T> Union<T> for Trait<'a, T> {}
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { dg-warning ".E0404." "" { target *-*-* } .-2 }
// { dg-warning ".E0404." "" { target *-*-* } .-3 }

fn main() {}

