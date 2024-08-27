// Regression test for issue #109299.

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Lexer<'d>(&'d ());

impl Lexer<'static> {
    type Cursor = ();
}

fn test(_: Lexer::Cursor) {} // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn main() {}

