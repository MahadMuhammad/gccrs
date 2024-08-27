//! This test used to ICE: rust-lang/rust#125914
//! Instead of actually analyzing the erroneous patterns,
//! we instead stop after typeck where errors are already
//! reported.

enum AstKind<'ast> {
// { dg-error ".E0392." "" { target *-*-* } .-1 }
    ExprInt,
}

enum Foo {
    Bar(isize),
    Baz,
}

enum Other {
    Other1(Foo),
    Other2(AstKind), // { dg-error ".E0106." "" { target *-*-* } }
}

fn main() {
    match Other::Other1(Foo::Baz) {
        ::Other::Other2(::Foo::Bar(..)) => {}
    }
}

