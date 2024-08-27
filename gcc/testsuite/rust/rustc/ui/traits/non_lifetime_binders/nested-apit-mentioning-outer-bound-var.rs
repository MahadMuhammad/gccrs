#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Trait<Input> {
    type Assoc;
}

fn uwu(_: impl for<T> Trait<(), Assoc = impl Trait<T>>) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

