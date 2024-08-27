// Tests to make sure that we reject polymorphic fn ptrs.

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn foo() -> for<T> fn(T) {
// { dg-error "" "" { target *-*-* } .-1 }
    todo!()
}

fn main() {
    foo()(1i32);
}

