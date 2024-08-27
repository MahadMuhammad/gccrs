// Regression test for issue #84195
// Checks that we properly fire lints that occur inside
// anon consts.

#![deny(semicolon_in_expressions_from_macros)]

macro_rules! len {
    () => { 0; }; // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
}

fn main() {
    let val: [u8; len!()] = [];
}

