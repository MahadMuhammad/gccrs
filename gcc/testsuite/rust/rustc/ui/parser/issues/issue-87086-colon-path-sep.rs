// Tests that a suggestion is issued if the user wrote a colon instead of
// a path separator in a match arm.

mod qux {
    pub enum Foo {
        Bar,
        Baz,
    }
}

use qux::Foo;

fn f() -> Foo { Foo::Bar }

fn g1() {
    match f() {
        Foo:Bar => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
    match f() {
        qux::Foo:Bar => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
    match f() {
        qux:Foo::Baz => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
    match f() {
        qux: Foo::Baz if true => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
    if let Foo:Bar = f() { // { dg-warning "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    }
}

fn g1_neg() {
    match f() {
        ref qux: Foo::Baz => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
}

fn g2_neg() {
    match f() {
        mut qux: Foo::Baz => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
}

fn main() {
    let myfoo = Foo::Bar;
    match myfoo {
        Foo::Bar => {}
        Foo:Bar::Baz => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match myfoo {
        Foo::Bar => {}
        Foo:Bar => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
}

