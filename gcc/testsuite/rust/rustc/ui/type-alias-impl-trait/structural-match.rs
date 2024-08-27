#![feature(type_alias_impl_trait)]

mod foo {
    pub type Foo = impl Send;

    // This is not structural-match
    struct A;

    pub const fn value() -> Foo {
        A
    }
}
use foo::*;
const VALUE: Foo = value();

fn test() {
    match VALUE {
        VALUE => (),
// { dg-error "" "" { target *-*-* } .-1 }
        _ => (),
    }
}

fn main() {}

