#![deny(refining_impl_trait)]

pub trait Foo {
    fn bar() -> impl Sized;
}

pub struct A;
impl Foo for A {
    fn bar() -> impl Copy {}
// { dg-error "" "" { target *-*-* } .-1 }
}

pub struct B;
impl Foo for B {
    fn bar() {}
// { dg-error "" "" { target *-*-* } .-1 }
}

pub struct C;
impl Foo for C {
    fn bar() -> () {}
// { dg-error "" "" { target *-*-* } .-1 }
}

struct Private;
impl Foo for Private {
    fn bar() -> () {}
// { dg-error "" "" { target *-*-* } .-1 }
}

pub trait Arg<A> {
    fn bar() -> impl Sized;
}
impl Arg<Private> for A {
    fn bar() -> () {}
// { dg-error "" "" { target *-*-* } .-1 }
}

pub trait Late {
    fn bar<'a>(&'a self) -> impl Sized + 'a;
}

pub struct D;
impl Late for D {
    fn bar(&self) -> impl Copy + '_ {}
// { dg-error "" "" { target *-*-* } .-1 }
}

mod unreachable {
    pub trait UnreachablePub {
        fn bar() -> impl Sized;
    }

    struct E;
    impl UnreachablePub for E {
        fn bar() {}
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

