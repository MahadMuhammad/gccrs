#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Lexer<'d>(&'d ());

impl Lexer<'d> { // { dg-error ".E0261." "" { target *-*-* } }
    type Cursor = ();
}

fn test(_: Lexer::Cursor) {}

fn main() {}

