// Regression test for #87573, ensures that duplicate lang items or invalid generics
// for lang items doesn't cause ICE.

#![feature(no_core, lang_items)]
#![no_core]
#![crate_type = "lib"]

pub static STATIC_BOOL: bool = true;

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

#[lang = "sync"]
trait Sync {}
impl Sync for bool {}

#[lang = "drop_in_place"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
fn drop_fn() {
    while false {}
}

#[lang = "start"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
fn start(){}

