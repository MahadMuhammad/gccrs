#![feature(specialization)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub trait Foo {
    fn foo();
}

impl Foo for i32 {}
impl Foo for i64 {
    fn foo() {}
// { dg-error ".E0520." "" { target *-*-* } .-1 }
}
impl<T> Foo for T {
    fn foo() {}
}

fn main() {
    i32::foo();
    i64::foo();
    u8::foo();
}

