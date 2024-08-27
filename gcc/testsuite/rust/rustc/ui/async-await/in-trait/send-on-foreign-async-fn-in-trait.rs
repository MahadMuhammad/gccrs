//@ aux-build:foreign-async-fn.rs
// { dg-additional-options "-frust-edition=2021" }

extern crate foreign_async_fn;
use foreign_async_fn::Foo;

fn bar<T: Foo>() {
    fn needs_send(_: impl Send) {}
    needs_send(T::test());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

