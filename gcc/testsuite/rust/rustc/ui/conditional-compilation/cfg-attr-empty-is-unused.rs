// Check that `#[cfg_attr($PREDICATE,)]` triggers the `unused_attribute` lint.

#![deny(unused)]

#[cfg_attr(FALSE,)] // { dg-error "" "" { target *-*-* } }
fn _f() {}

#[cfg_attr(not(FALSE),)] // { dg-error "" "" { target *-*-* } }
fn _g() {}

fn main() {}

