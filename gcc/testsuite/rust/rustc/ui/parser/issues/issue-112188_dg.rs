//@ run-rustfix

#![allow(unused)]

struct Foo { x: i32 }

fn main() {
    let f = Foo { x: 0 };
    let Foo { .. } = f;
    let Foo { .., } = f; // { dg-error "" "" { target *-*-* } }
    let Foo { x, .. } = f;
    let Foo { .., x } = f; // { dg-error "" "" { target *-*-* } }
    let Foo { .., x, .. } = f; // { dg-error "" "" { target *-*-* } }
}

