#![deny(non_upper_case_globals)]

trait Trait {
    const item: usize;
// { dg-error "" "" { target *-*-* } .-1 }
}

struct Foo;

impl Trait for Foo {
    const item: usize = 5;
    // ^^^ there should be no error here (in the trait `impl`)
}

fn main() {}

