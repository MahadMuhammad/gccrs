// Allow an unstable re-export without requiring a feature gate.
// #94972

//@ aux-build:lint-stability.rs
//@ aux-build:lint-stability-reexport.rs
#![feature(staged_api)]
#![stable(feature = "lint_stability", since = "1.0.0")]

extern crate lint_stability;
extern crate lint_stability_reexport;

#[unstable(feature = "unstable_test_feature", issue = "none")]
pub use lint_stability::unstable;

// We want to confirm that using a re-export through another crate behaves
// the same way as using an item directly
#[unstable(feature = "unstable_test_feature", issue = "none")]
pub use lint_stability_reexport::unstable_text;

// Ensure items which aren't marked as unstable can't re-export unstable items
#[stable(feature = "lint_stability", since = "1.0.0")]
pub use lint_stability::unstable as unstable2;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {
    // Since we didn't enable the feature in this crate, we still can't
    // use these items, even though they're in scope from the `use`s which are now allowed.
    unstable(); // { dg-error ".E0658." "" { target *-*-* } }
    unstable_text(); // { dg-error ".E0658." "" { target *-*-* } }
}

