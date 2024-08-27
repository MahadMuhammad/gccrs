//@ compile-flags: -Znext-solver

#![feature(pointer_like_trait)]

use std::marker::PointerLike;

fn require_(_: impl PointerLike) {}

fn main() {
    require_(1usize);
    require_(1u16);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    require_(&1i16);
}

