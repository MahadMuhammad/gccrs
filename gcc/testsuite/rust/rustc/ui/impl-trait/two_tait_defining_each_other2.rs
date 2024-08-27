//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
#![feature(type_alias_impl_trait)]

type A = impl Foo; // { dg-error "" "" { target *-*-* } }
type B = impl Foo;

trait Foo {}

fn muh(x: A) -> B {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    x // B's hidden type is A (opaquely)
// { dg-error "" "" { target *-*-* } .-1 }
}

struct Bar;
impl Foo for Bar {}

fn main() {}

