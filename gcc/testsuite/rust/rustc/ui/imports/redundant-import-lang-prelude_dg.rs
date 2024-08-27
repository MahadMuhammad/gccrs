// Check that we detect imports that are redundant due to the language prelude
// and that we emit a reasonable diagnostic.
// { dg-note "" "" { target *-*-* } .-2 }

// Note that we use the term "extern prelude" in the label even though "language prelude"
// would be more correct. However, it's not worth special-casing this.

// See also the discussion in <https://github.com/rust-lang/rust/pull/122954>.

#![deny(redundant_imports)]
// { dg-note "" "" { target *-*-* } .-1 }

use std::primitive::u8;
// { dg-error "" "" { target *-*-* } .-1 }

const _: u8 = 0;

fn main() {}

