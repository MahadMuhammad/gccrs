// Test that Fn-family traits with lifetime parameters shouldn't compile and
// we suggest the usage of higher-rank trait bounds instead.

fn fa(_: impl Fn<'a>(&'a str) -> bool) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn fb(_: impl FnMut<'a, 'b>(&'a str, &'b str) -> bool) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn fc(_: impl std::fmt::Display + FnOnce<'a>(&'a str) -> bool + std::fmt::Debug) {}
// { dg-error "" "" { target *-*-* } .-1 }

use std::ops::Fn as AliasedFn;
fn fd(_: impl AliasedFn<'a>(&'a str) -> bool) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn fe<F>(_: F) where F: Fn<'a>(&'a str) -> bool {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

