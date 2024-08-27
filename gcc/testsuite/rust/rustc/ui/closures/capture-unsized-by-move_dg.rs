//@ compile-flags: --crate-type=lib

#![feature(unsized_fn_params)]

pub fn f(k: dyn std::fmt::Display) {
    let k2 = move || {
        k.to_string();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    };
}

