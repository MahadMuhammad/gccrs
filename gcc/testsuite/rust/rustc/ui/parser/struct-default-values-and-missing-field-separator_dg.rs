//@ run-rustfix
#![allow(dead_code)]

enum E {
    A,
}

struct S {
    field1: i32 = 42, // { dg-error "" "" { target *-*-* } }
    field2: E = E::A, // { dg-error "" "" { target *-*-* } }
    field3: i32 = 1 + 2, // { dg-error "" "" { target *-*-* } }
    field4: i32 = { 1 + 2 }, // { dg-error "" "" { target *-*-* } }
    field5: E = foo(42), // { dg-error "" "" { target *-*-* } }
    field6: E = { foo(42) }, // { dg-error "" "" { target *-*-* } }
}

struct S1 {
    field1: i32 // { dg-error "" "" { target *-*-* } }
    field2: E // { dg-error "" "" { target *-*-* } }
    field3: i32 = 1 + 2, // { dg-error "" "" { target *-*-* } }
    field4: i32 = { 1 + 2 }, // { dg-error "" "" { target *-*-* } }
    field5: E = foo(42), // { dg-error "" "" { target *-*-* } }
    field6: E = { foo(42) }, // { dg-error "" "" { target *-*-* } }
}

struct S2 {
    field1 = i32, // { dg-error "" "" { target *-*-* } }
    field2; E, // { dg-error "" "" { target *-*-* } }
}

const fn foo(_: i32) -> E {
    E::A
}

fn main() {}

