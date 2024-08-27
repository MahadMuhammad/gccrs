// Checks that we do not ICE when comparing `Self` to `Pin`
// { dg-additional-options "-frust-edition=2021" }

struct S;

impl S {
    fn foo(_: Box<Option<S>>) {}
    fn bar() {
        Self::foo(None) // { dg-error ".E0308." "" { target *-*-* } }
    }
}

fn main() {}

