#![feature(type_alias_impl_trait)]

type Foo = impl for<'a> FnOnce(&'a str) -> usize;
type Bar = impl FnOnce(&'static str) -> usize;

fn foo() -> Foo {
    if true {
        |s| s.len() // { dg-error ".E0282." "" { target *-*-* } }
    } else {
        panic!()
    }
}
fn bar() -> Bar {
    if true {
        |s| s.len() // { dg-error ".E0282." "" { target *-*-* } }
    } else {
        panic!()
    }
}

fn foo2() -> impl for<'a> FnOnce(&'a str) -> usize {
    if true {
        |s| s.len() // { dg-error ".E0282." "" { target *-*-* } }
    } else {
        panic!()
    }
}
fn bar2() -> impl FnOnce(&'static str) -> usize {
    if true {
        |s| s.len() // { dg-error ".E0282." "" { target *-*-* } }
    } else {
        panic!()
    }
}

fn main() {}

