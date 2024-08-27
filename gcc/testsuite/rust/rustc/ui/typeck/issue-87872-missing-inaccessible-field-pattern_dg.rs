#![allow(dead_code, unused_variables)]

pub mod foo {
    #[derive(Default)]
    pub struct Foo { pub visible: bool, invisible: bool, }
}

fn main() {
    let foo::Foo {} = foo::Foo::default();
// { dg-error ".E0027." "" { target *-*-* } .-1 }
}

