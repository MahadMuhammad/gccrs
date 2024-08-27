// run-check
//@ aux-build:proc-macro-type-error.rs

extern crate proc_macro_type_error;

use proc_macro_type_error::hello;

#[hello] // { dg-error ".E0308." "" { target *-*-* } }
fn abc() {}

fn x(_: &mut i32) {}

macro_rules! bla {
    () => {
        x(123);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { suggestion ".E0308." "" { target *-*-* } .-2 }
    };
    ($v:expr) => {
        x($v)
    }
}

fn main() {
    bla!();
    bla!(456);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { suggestion ".E0308." "" { target *-*-* } .-2 }
}

