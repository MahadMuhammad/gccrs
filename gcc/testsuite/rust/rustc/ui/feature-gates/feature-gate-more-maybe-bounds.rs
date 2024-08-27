#![feature(auto_traits)]

trait Trait1 {}
auto trait Trait2 {}
trait Trait3: ?Trait1 {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
trait Trait4 where Self: ?Trait1 {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn foo(_: Box<dyn Trait1 + ?Trait2>) {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar<T: ?Trait1 + ?Trait2>(_: T) {}
// { dg-error ".E0203." "" { target *-*-* } .-1 }
// { dg-warning ".E0203." "" { target *-*-* } .-2 }
// { dg-warning ".E0203." "" { target *-*-* } .-3 }

trait Trait {}
// Do not suggest `#![feature(more_maybe_bounds)]` for repetitions
fn baz<T: ?Trait + ?Trait>(_ : T) {}
// { dg-error ".E0203." "" { target *-*-* } .-1 }
// { dg-warning ".E0203." "" { target *-*-* } .-2 }
// { dg-warning ".E0203." "" { target *-*-* } .-3 }

fn main() {}

