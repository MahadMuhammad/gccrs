#![allow(unused, nonstandard_style)]

// If an enum has two different variants,
// then it cannot be matched upon in a function argument.
// It still gets an error, but no suggestions.
enum Foo {
    C,
    D,
}

fn foo(C: Foo) {} // { dg-error ".E0170." "" { target *-*-* } }

fn main() {
    let C = Foo::D; // { dg-error ".E0170." "" { target *-*-* } }
}

