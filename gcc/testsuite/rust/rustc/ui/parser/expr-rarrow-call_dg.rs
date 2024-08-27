//@ run-rustfix
#![allow(
    dead_code,
    unused_must_use
)]

struct Named {
    foo: usize
}

struct Unnamed(usize);

fn named_struct_field_access(named: &Named) {
    named->foo; // { dg-error "" "" { target *-*-* } }
}

fn unnamed_struct_field_access(unnamed: &Unnamed) {
    unnamed->0; // { dg-error "" "" { target *-*-* } }
}

fn tuple_field_access(t: &(u8, u8)) {
    t->0; // { dg-error "" "" { target *-*-* } }
    t->1; // { dg-error "" "" { target *-*-* } }
}

#[derive(Clone)]
struct Foo;

fn method_call(foo: &Foo) {
    foo->clone(); // { dg-error "" "" { target *-*-* } }
}

fn main() {}

