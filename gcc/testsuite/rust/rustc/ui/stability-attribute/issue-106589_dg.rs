// #![feature(staged_api)] // note: `staged_api` not enabled

#![stable(feature = "foo", since = "1.0.0")]
// { dg-error ".E0734." "" { target *-*-* } .-1 }

#[unstable(feature = "foo", issue = "none")]
// { dg-error ".E0734." "" { target *-*-* } .-1 }
fn foo_unstable() {}

fn main() {}

