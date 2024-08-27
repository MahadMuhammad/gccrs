//@ revisions: default feature
#![cfg_attr(feature, feature(arbitrary_self_types))]

use std::ops::Deref;

struct Foo(u32);
impl Foo {
    fn get<R: Deref<Target = Self>>(self: R) -> u32 {
// { dg-error "" "" { target *-*-* } .-1 }
        self.0
    }
}

fn main() {
    let mut foo = Foo(1);
    foo.get::<&Foo>();
// { dg-error "" "" { target *-*-* } .-1 }
}

