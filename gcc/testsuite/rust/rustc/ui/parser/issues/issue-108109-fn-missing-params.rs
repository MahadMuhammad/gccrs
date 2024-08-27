//@ run-rustfix

pub fn missing -> () {}
// { dg-error "" "" { target *-*-* } .-1 }

pub fn missing2 {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

