//@ check-pass

#![feature(trait_alias)]

trait Foo = std::fmt::Display + std::fmt::Debug;
trait bar = std::fmt::Display + std::fmt::Debug; // { dg-warning "" "" { target *-*-* } }

fn main() {}

