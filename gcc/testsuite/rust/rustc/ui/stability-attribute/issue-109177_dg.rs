//@ aux-build: similar-unstable-method.rs

extern crate similar_unstable_method;

fn main() {
    // FIXME: this function should not suggest the `foo` function.
    similar_unstable_method::foo1();
// { dg-error ".E0425." "" { target *-*-* } .-1 }

    let foo = similar_unstable_method::Foo;
    foo.foo1();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

