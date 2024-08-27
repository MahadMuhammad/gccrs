// Regression test for #110899.
// When looking for the elided lifetime for `wrap`,
// we must not consider the lifetimes in `bar` as candidates.

fn wrap(self: Wrap<{ fn bar(&self) {} }>) -> &() {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
// { dg-error ".E0412." "" { target *-*-* } .-3 }
// { dg-error ".E0412." "" { target *-*-* } .-4 }
    &()
}

fn main() {}

