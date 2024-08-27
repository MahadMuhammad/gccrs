//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
// { dg-additional-options "-frust-edition=2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Foo {
    async fn bar(&self);
}

struct Bar;
impl Foo for Bar {
    async fn bar(&self) {}
}

fn build<T>(_: T) where T: Foo<bar(..): Send> {}

fn main() {
    build(Bar);
}

