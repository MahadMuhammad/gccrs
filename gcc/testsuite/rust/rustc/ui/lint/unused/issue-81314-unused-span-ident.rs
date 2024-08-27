//@ run-rustfix
// Regression test for #81314: Unused variable lint should
// span only the identifier and not the rest of the pattern

#![deny(unused)]

fn main() {
    let [rest @ ..] = [1, 2, 3]; // { dg-error "" "" { target *-*-* } }
}

pub fn foo([rest @ ..]: &[i32]) { // { dg-error "" "" { target *-*-* } }
}

