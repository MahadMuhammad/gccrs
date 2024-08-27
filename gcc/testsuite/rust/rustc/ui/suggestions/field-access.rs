//@ run-rustfix
#![allow(dead_code)]

struct A {
    b: B,
}

enum B {
    Fst,
    Snd,
}

union Foo {
    bar: u32,
    qux: f32,
}

fn main() {
    let a = A { b: B::Fst };
    if let B::Fst = a {}; // { dg-error ".E0308." "" { target *-*-* } }
// { help ".E0308." "" { target *-*-* } .-1 }
    match a {
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        B::Fst => (), // { dg-error ".E0308." "" { target *-*-* } }
        B::Snd => (), // { dg-error ".E0308." "" { target *-*-* } }
    }

    let foo = Foo { bar: 42 };
    match foo {
// { help "" "" { target *-*-* } .-1 }
        1u32 => (), // { dg-error ".E0308." "" { target *-*-* } }
        _ => (),
    }
}

