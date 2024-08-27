//@ run-pass

#![allow(dead_code)]

struct Foo {
    foo: i32,
    bar: (),
    baz: (),
}

fn use_foo(x: Foo) -> i32 {
    let Foo { foo, bar, .. } = x; // { dg-warning "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
    return foo;
}

// issue #105028, suggest removing the field only for shorthand
fn use_match(x: Foo) {
    match x {
        Foo { foo: unused, .. } => { // { dg-warning "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
        }
    }

    match x {
        Foo { foo, .. } => { // { dg-warning "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
        }
    }
}

fn main() {}

