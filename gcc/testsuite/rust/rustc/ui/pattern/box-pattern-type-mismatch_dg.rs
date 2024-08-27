//! This test used to ICE #124004

#![feature(box_patterns)]

use std::ops::{ Deref };

struct X(dyn Iterator<Item = &'a ()>);
// { dg-error ".E0261." "" { target *-*-* } .-1 }

impl Deref for X {
    type Target = isize;

    fn deref(&self) -> &isize {
        let &X(box ref x) = self;
        x
    }
}

fn main() {}

