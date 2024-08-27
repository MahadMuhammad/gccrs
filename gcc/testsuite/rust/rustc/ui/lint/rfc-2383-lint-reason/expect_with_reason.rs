//@ check-pass

#![warn(unused)]

#![expect(unused_variables, reason = "<This should fail and display this reason>")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }

fn main() {}

