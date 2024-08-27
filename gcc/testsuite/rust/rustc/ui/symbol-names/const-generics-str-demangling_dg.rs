//@ build-fail
//@ compile-flags: -C symbol-mangling-version=v0 --crate-name=c
//@ normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
#![feature(adt_const_params, unsized_const_params, rustc_attrs)]
#![allow(incomplete_features)]

pub struct Str<const S: &'static str>;

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Str<"abc"> {}

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Str<"'"> {}

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Str<"\t\n"> {}

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Str<"∂ü"> {}

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
impl Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜"> {}

fn main() {}

