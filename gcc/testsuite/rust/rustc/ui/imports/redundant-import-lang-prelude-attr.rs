// Check that we detect imports (of built-in attributes) that are redundant due to
// the language prelude and that we emit a reasonable diagnostic.
// { dg-note "" "" { target *-*-* } .-2 }

// Note that we use the term "extern prelude" in the label even though "language prelude"
// would be more correct. However, it's not worth special-casing this.

// See also the discussion in <https://github.com/rust-lang/rust/pull/122954>.

// { dg-additional-options "-frust-edition= 2018" }

#![deny(redundant_imports)]
// { dg-note "" "" { target *-*-* } .-1 }

use allow; // { dg-error "" "" { target *-*-* } }

#[allow(unused)]
fn main() {}

