// { dg-additional-options "-frust-edition= 2021" }
//@ run-rustfix
//@ rustfix-only-machine-applicable
//@ aux-build:match_ergonomics_2024_macros.rs
#![feature(mut_ref, ref_pat_eat_one_layer_2024)]
#![allow(incomplete_features, unused)]
#![deny(rust_2024_incompatible_pat)]

extern crate match_ergonomics_2024_macros;

struct Foo(u8);

fn main() {
    let Foo(mut a) = &Foo(0);
// { dg-error "" "" { target *-*-* } .-1 }
    a = 42;

    let Foo(mut a) = &mut Foo(0);
// { dg-error "" "" { target *-*-* } .-1 }
    a = 42;

    if let Some(&_) = &&&&&Some(&0u8) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(&mut _) = &&&&&Some(&mut 0u8) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(&_) = &&&&&mut Some(&0u8) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(&mut Some(Some(_))) = &mut Some(&mut Some(&mut Some(0u8))) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(&mut Some(Some(_a))) = &mut Some(&mut Some(&mut Some(0u8))) {}
// { dg-error "" "" { target *-*-* } .-1 }

    struct Struct {
        a: u32,
        b: u32,
        c: u32,
    }
    let s = Struct { a: 0, b: 0, c: 0 };
    let Struct { a, mut b, c } = &s;
// { dg-error "" "" { target *-*-* } .-1 }

    #[warn(rust_2024_incompatible_pat)]
    match &(Some(0), Some(0)) {
        // The two patterns are the same syntactically, but because they're defined in different
        // editions they don't mean the same thing.
        (Some(mut _x), match_ergonomics_2024_macros::mixed_edition_pat!(_y)) => {
// { dg-warning "" "" { target *-*-* } .-1 }
            _x = 4;
            _y = &7;
        }
        _ => {}
    }
}

