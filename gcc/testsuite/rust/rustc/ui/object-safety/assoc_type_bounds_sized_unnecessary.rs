//@ check-pass

trait Foo {
    type Bar
    where
        Self: Sized;
}

fn foo(_: &dyn Foo<Bar = ()>) {}
// { dg-warning "" "" { target *-*-* } .-1 }

#[allow(unused_associated_type_bounds)]
fn bar(_: &dyn Foo<Bar = ()>) {}

fn main() {}

