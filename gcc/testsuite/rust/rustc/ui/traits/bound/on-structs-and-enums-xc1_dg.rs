//@ aux-build:on_structs_and_enums_xc.rs

extern crate on_structs_and_enums_xc;

use on_structs_and_enums_xc::{Bar, Foo, Trait};

fn main() {
    let foo = Foo {
        x: 3
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    };
    let bar: Bar<f64> = return;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let _ = bar;
}

