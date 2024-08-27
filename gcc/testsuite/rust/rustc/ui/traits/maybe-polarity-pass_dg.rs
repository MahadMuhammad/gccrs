//@ check-pass

#![feature(auto_traits)]
#![feature(more_maybe_bounds)]
#![feature(negative_impls)]

trait Trait1 {}
auto trait Trait2 {}

trait Trait3 : ?Trait1 {}
trait Trait4 where Self: Trait1 {}

fn foo(_: Box<(dyn Trait3 + ?Trait2)>) {}
fn bar<T: ?Sized + ?Trait2 + ?Trait1 + ?Trait4>(_: &T) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }

struct S;
impl !Trait2 for S {}
impl Trait1 for S {}
impl Trait3 for S {}

fn main() {
    foo(Box::new(S));
    bar(&S);
}

