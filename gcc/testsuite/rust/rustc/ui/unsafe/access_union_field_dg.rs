#![allow(unused_variables)]

union Foo {
    bar: i8,
    baz: u8,
}

fn main() {
    let foo = Foo { bar: 5 };
    let a = foo.bar; // { dg-error ".E0133." "" { target *-*-* } }
    let b = foo.baz; // { dg-error ".E0133." "" { target *-*-* } }
}

