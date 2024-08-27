//@ run-rustfix
#![allow(dead_code)]
mod first {
    trait Foo { fn m(self: Box<Self>); }
    fn foo<T: Foo>(a: T) {
        a.m(); // { dg-error ".E0599." "" { target *-*-* } }
    }
}
mod second {
    use std::sync::Arc;
    trait Bar { fn m(self: Arc<Self>); }
    fn bar(b: impl Bar) {
        b.m(); // { dg-error ".E0599." "" { target *-*-* } }
    }
}

fn main() {}

