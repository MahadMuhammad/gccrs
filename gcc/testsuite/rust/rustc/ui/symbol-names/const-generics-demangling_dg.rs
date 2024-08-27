//@ build-fail
//@ revisions: legacy v0
//@ compile-flags: --crate-name=c
//@[legacy]compile-flags: -C symbol-mangling-version=legacy -Z unstable-options
//@    [v0]compile-flags: -C symbol-mangling-version=v0
//@[legacy]normalize-stderr-test: "h[[:xdigit:]]{16}" -> "h[HASH]"
//@    [v0]normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
#![feature(rustc_attrs)]

pub struct Unsigned<const F: u8>;

impl Unsigned<11> {
    #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    fn f() {}
}

pub struct Signed<const F: i16>;

impl Signed<-152> {
    #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    fn f() {}
}

pub struct Bool<const F: bool>;

impl Bool<true> {
    #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    fn f() {}
}

pub struct Char<const F: char>;

impl Char<'∂'> {
    #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    fn f() {}
}

fn main() {}

