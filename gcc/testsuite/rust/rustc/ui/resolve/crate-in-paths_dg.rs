// { dg-additional-options "-frust-edition=2018" }

mod bar {
    pub(crate) struct Foo;
}

fn main() {
    Foo;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

