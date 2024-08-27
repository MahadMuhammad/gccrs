// Regression test for #113265.

// Don't perform selection if the feature is not enabled to prevent cycle errors
// that exist due to current limitations of the implementation from masking the
// feature-gate error. See the aforementioned issue.
// This does lead to rustc not mentioning inherent associated types at usage-sites of
// IATs that were defined in an external crate but that's acceptable for now.

// FIXME(inherent_associated_types): Revisit this decision once the implementation is smarter.

// The following program would currently lead to a cycle if IATs were enabled.

struct S(S::P); // { dg-error ".E0223." "" { target *-*-* } }

impl S { type P = (); } // { dg-error ".E0658." "" { target *-*-* } }

fn main() {}

