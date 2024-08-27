// Unresolved fields are not copy, but also shouldn't report an extra E0740.

pub union Foo {
    x: *const Missing,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

fn main() {}

