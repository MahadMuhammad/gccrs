// Tests to make sure that we reject polymorphic dyn trait.

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Test<T> {}

fn foo() -> &'static dyn for<T> Test<T> {
// { dg-error "" "" { target *-*-* } .-1 }
    todo!()
}

fn main() {}

