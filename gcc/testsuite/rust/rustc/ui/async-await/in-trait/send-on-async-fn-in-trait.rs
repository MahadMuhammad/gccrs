//@ run-rustfix
// { dg-additional-options "-frust-edition= 2021" }

#![allow(unused)]

trait Foo {
    async fn test() -> () {}
    async fn test2() -> i32 { 1 + 2 }
}

fn bar<T: Foo>() {
    fn needs_send(_: impl Send) {}
    needs_send(T::test());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    needs_send(T::test2());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

