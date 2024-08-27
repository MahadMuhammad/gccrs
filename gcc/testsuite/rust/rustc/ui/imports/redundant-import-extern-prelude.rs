// Check that we detect imports that are redundant due to the extern prelude
// and that we emit a reasonable diagnostic.
// issue: rust-lang/rust#121915
// { dg-note "" "" { target *-*-* } .-3 }

// See also the discussion in <https://github.com/rust-lang/rust/pull/122954>.

//@ compile-flags: --extern aux_issue_121915 --edition 2018
//@ aux-build: aux-issue-121915.rs

#[deny(redundant_imports)]
// { dg-note "" "" { target *-*-* } .-1 }
fn main() {
    use aux_issue_121915;
// { dg-error "" "" { target *-*-* } .-1 }
    aux_issue_121915::item();
}

