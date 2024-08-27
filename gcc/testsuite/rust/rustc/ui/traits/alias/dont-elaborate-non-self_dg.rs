#![feature(trait_alias)]

use std::future::Future;

trait F<Fut: Future<Output = usize>> = Fn() -> Fut;

fn f<Fut>(a: dyn F<Fut>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}

