//@ run-rustfix

pub fn func<F>() where F: FnOnce -> () {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }

fn main() {}

