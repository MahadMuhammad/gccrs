#![feature(more_maybe_bounds)]

trait Trait {}
fn foo<T: ?Trait + ?Trait>(_: T) {}
// { dg-error ".E0203." "" { target *-*-* } .-1 }
// { dg-warning ".E0203." "" { target *-*-* } .-2 }
// { dg-warning ".E0203." "" { target *-*-* } .-3 }

fn main() {}

