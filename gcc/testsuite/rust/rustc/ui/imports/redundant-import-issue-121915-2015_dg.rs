//@ compile-flags: --extern aux_issue_121915 --edition 2015
//@ aux-build: aux-issue-121915.rs

extern crate aux_issue_121915;

#[deny(redundant_imports)]
fn main() {
    use aux_issue_121915;
// { dg-error "" "" { target *-*-* } .-1 }
    aux_issue_121915::item();
}

