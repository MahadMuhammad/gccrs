//@ check-pass

#![feature(dyn_star)]
// { dg-warning "" "" { target *-*-* } .-1 }

const C: dyn* Send + Sync = &();

static S: dyn* Send + Sync = &();

fn main() {}

