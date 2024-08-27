//@ aux-build:on_structs_and_enums_xc.rs

extern crate on_structs_and_enums_xc;

use on_structs_and_enums_xc::{Bar, Foo, Trait};

fn explode(x: Foo<usize>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn kaboom(y: Bar<f32>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {
}

