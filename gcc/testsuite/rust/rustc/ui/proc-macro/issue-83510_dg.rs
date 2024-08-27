//@ aux-build: issue-83510.rs

extern crate issue_83510;

issue_83510::dance_like_you_want_to_ice!();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }

fn main() {}

