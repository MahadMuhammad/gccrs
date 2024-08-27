#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct ConstAssert<const COND: bool>;
trait True {}
impl True for ConstAssert<true> {}

struct Range<T: PartialOrd, const MIN: T, const MAX: T>(T)
// { dg-error ".E0770." "" { target *-*-* } .-1 }
// { dg-error ".E0770." "" { target *-*-* } .-2 }
where
    ConstAssert<{ MIN <= MAX }>: True;

fn main() {}

