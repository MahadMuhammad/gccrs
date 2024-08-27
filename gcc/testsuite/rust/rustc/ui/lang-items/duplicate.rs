//@ normalize-stderr-test: "loaded from .*libcore-.*.rlib" -> "loaded from SYSROOT/libcore-*.rlib"
#![feature(lang_items)]

#[lang = "sized"]
trait Sized {}
// { dg-error ".E0152." "" { target *-*-* } .-1 }

#[lang = "tuple_trait"]
pub trait Tuple {}
// no error

