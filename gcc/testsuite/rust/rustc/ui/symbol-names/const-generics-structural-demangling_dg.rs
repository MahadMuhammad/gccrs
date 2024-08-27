//@ build-fail
//@ compile-flags: -C symbol-mangling-version=v0 --crate-name=c

//@ normalize-stderr-test: "c\[[0-9a-f]+\]" -> "c[HASH]"

#![feature(adt_const_params, unsized_const_params, decl_macro, rustc_attrs)]
#![allow(incomplete_features)]

use std::marker::UnsizedConstParamTy;

pub struct RefByte<const RB: &'static u8>;

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl RefByte<{ &123 }> {}

// FIXME(eddyb) this was supposed to be `RefMutZst` with `&mut []`,
// but that is currently not allowed in const generics.
pub struct RefZst<const RMZ: &'static [u8; 0]>;

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl RefZst<{ &[] }> {}

pub struct Array3Bytes<const A3B: [u8; 3]>;

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Array3Bytes<{ [1, 2, 3] }> {}

pub struct TupleByteBool<const TBB: (u8, bool)>;

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl TupleByteBool<{ (1, false) }> {}

#[derive(PartialEq, Eq, UnsizedConstParamTy)]
pub enum MyOption<T> {
    Some(T),
    None,
}

pub struct OptionUsize<const OU: MyOption<usize>>;

// HACK(eddyb) the full mangling is only in `.stderr` because we can normalize
// the `core` disambiguator hash away there, but not here.
#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl OptionUsize<{ MyOption::None }> {}

// HACK(eddyb) the full mangling is only in `.stderr` because we can normalize
// the `core` disambiguator hash away there, but not here.
#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl OptionUsize<{ MyOption::Some(0) }> {}

#[derive(PartialEq, Eq, UnsizedConstParamTy)]
pub struct Foo {
    s: &'static str,
    ch: char,
    slice: &'static [u8],
}
pub struct Foo_<const F: Foo>;

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Foo_<{ Foo { s: "abc", ch: 'x', slice: &[1, 2, 3] } }> {}

// NOTE(eddyb) this tests specifically the use of disambiguators in field names,
// using macros 2.0 hygiene to create a `struct` with conflicting field names.
macro duplicate_field_name_test($x:ident) {
    #[derive(PartialEq, Eq, UnsizedConstParamTy)]
    pub struct Bar {
        $x: u8,
        x: u16,
    }
    pub struct Bar_<const B: Bar>;

    #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    impl Bar_<{ Bar { $x: 123, x: 4096 } }> {}
}
duplicate_field_name_test!(x);

fn main() {}

