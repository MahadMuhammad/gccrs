//@ build-fail
#![feature(rustc_attrs)]

#[rustc_dump_vtable]
trait A {
    fn foo_a(&self) {}
}

#[rustc_dump_vtable]
trait B {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_b(&self) {}
}

#[rustc_dump_vtable]
trait C: A + B {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_c(&self) {}
}

struct S;

impl A for S {}
impl B for S {}
impl C for S {}

fn foo(c: &dyn C) {}
fn bar(c: &dyn B) {}

fn main() {
    foo(&S);
    bar(&S);
}

