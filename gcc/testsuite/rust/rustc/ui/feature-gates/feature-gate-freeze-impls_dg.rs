#![feature(freeze, negative_impls)]

use std::marker::Freeze;

struct Foo;

unsafe impl Freeze for Foo {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

struct Bar;

impl !Freeze for Bar {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

