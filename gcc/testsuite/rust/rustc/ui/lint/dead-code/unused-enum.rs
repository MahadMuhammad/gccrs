#![deny(unused)]

struct F; // { dg-error "" "" { target *-*-* } }
struct B; // { dg-error "" "" { target *-*-* } }

enum E {
// { dg-error "" "" { target *-*-* } .-1 }
    Foo(F),
    Bar(B),
}

fn main() {}

