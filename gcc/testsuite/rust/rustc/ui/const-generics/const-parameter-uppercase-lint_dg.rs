#![deny(non_upper_case_globals)]

fn noop<const x: u32>() {
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

