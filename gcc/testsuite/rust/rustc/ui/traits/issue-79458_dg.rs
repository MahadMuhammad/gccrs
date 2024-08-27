// Negative implementations should not be shown in trait suggestions.
// This is a regression test of #79458.

#[derive(Clone)]
struct Foo<'a, T> {
    bar: &'a mut T
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

