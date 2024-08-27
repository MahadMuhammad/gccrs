#![deny(unknown_lints)]

#![allow(not_a_real_lint)] // { dg-error "" "" { target *-*-* } }

#![deny(dead_cod)] // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }

#![deny(rust_2018_idiots)] // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }

fn main() {}

