//@ revisions: normal exhaustive_patterns
#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![feature(never_type)]
#![deny(unreachable_patterns)]

fn main() {}

fn foo(nevers: &[!]) {
    match nevers {
// { dg-error "" "" { target *-*-* } .-1 }
        &[] => (),
    };

    match nevers {
        &[] => (),
        &[_] => (),
        &[_, _, ..] => (),
    };

    match nevers {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        &[_] => (),
    };
}

