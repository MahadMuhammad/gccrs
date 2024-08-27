//@ run-rustfix

pub trait Trait {}

pub struct Foo;

impl Trait for Foo {}

fn foo<'x, P>(
    _post: P,
    x: &'x Foo,
) -> &'x impl Trait {
    x
}

pub fn bar<'t, T>(
// { help "" "" { target *-*-* } .-1 }
    post: T,
    x: &'t Foo,
) -> &'t impl Trait {
    foo(post, x)
// { dg-error ".E0309." "" { target *-*-* } .-1 }
}

fn main() {}

