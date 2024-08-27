#![feature(effects)]
// { dg-warning "" "" { target *-*-* } .-1 }

struct A();

impl const Drop for A {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
// { dg-error ".E0046." "" { target *-*-* } .-3 }

fn main() {}

