// --force-warn $LINT_GROUP causes $LINT to warn despite $LINT being
// allowed in module and cap-lints set to warn
//@ compile-flags: --cap-lints warn  --force-warn rust-2021-compatibility
//@ check-pass
#![allow(ellipsis_inclusive_range_patterns)]

pub fn f() -> bool {
    let x = 123;
    match x {
        0...100 => true,
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        _ => false,
    }
}

fn main() {}

