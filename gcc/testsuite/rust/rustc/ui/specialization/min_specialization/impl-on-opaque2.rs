// Test that specializing on opaque types is allowed

#![feature(min_specialization, type_alias_impl_trait)]

trait SpecTrait<U, V> {
    fn f();
}

impl<U> SpecTrait<U, ()> for () {
    default fn f() {}
}

type Opaque = impl Tuple;

trait Tuple {}

impl Tuple for () {}

// FIXME: this passes if we use `<(), ()>` here instead of `<(), Opaque>`,
// even though there can't be more overlap from the opaque version
impl SpecTrait<(), Opaque> for () {
// { dg-error ".E0119." "" { target *-*-* } .-1 }
    fn f() {}
}

fn foo() -> Opaque {}

fn main() {}

