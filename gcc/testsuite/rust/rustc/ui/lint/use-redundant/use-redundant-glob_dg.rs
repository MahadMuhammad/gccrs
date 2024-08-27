//@ check-pass
#![warn(redundant_imports)]

pub mod bar {
    pub struct Foo(pub Bar);
    pub struct Bar(pub char);
}

pub fn warning() -> bar::Foo {
    use bar::*;
    use bar::Foo; // { dg-warning "" "" { target *-*-* } }
    Foo(Bar('a'))
}

fn main() {}

