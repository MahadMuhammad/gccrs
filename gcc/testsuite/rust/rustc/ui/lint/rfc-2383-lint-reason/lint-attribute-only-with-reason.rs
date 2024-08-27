#![deny(unused_attributes)]

#[allow(reason = "I want to allow something")]// { dg-error "" "" { target *-*-* } }
#[expect(reason = "I don't know what I'm waiting for")]// { dg-error "" "" { target *-*-* } }
#[warn(reason = "This should be warn by default")]// { dg-error "" "" { target *-*-* } }
#[deny(reason = "All listed lints are denied")]// { dg-error "" "" { target *-*-* } }
#[forbid(reason = "Just some reason")]// { dg-error "" "" { target *-*-* } }

#[allow(clippy::box_collection, reason = "This is still valid")]
#[warn(dead_code, reason = "This is also reasonable")]

fn main() {}

