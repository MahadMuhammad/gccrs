// Test for #95898: The trait suggestion had an extra `:` after the trait.
// { dg-additional-options "-frust-edition=2021" }

fn foo<T:>(t: T) {
    t.clone();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {}

