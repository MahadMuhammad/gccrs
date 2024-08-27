use std::ops::Deref;

struct Foo;

impl<Foo> Deref for Foo { } // { dg-error ".E0210." "" { target *-*-* } }

fn main() {}

