// { dg-additional-options "-frust-edition=2018" }

default pub const async unsafe extern fn err() {} // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

pub default const async unsafe extern fn ok() {}

