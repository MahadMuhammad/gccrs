//@ run-rustfix

#![allow(unused)]

struct Foo<T>(T);

impl<T> Foo<T> {
    fn test() -> i32 { 1 }
}

fn main() {
    let x = Box::new(Foo(1i32));
    x.test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

