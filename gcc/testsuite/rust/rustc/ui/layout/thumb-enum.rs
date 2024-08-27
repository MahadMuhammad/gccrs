//@ compile-flags: --target thumbv8m.main-none-eabihf
//@ needs-llvm-components: arm
//
// Verify that thumb targets implement the repr(C) for enums correctly.
//
// See #87917
#![feature(never_type, rustc_attrs, no_core, lang_items)]
#![crate_type = "lib"]
#![no_core]

#[lang="sized"]
trait Sized {}

#[rustc_layout(debug)]
#[repr(C)]
enum A { Apple } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
#[repr(C)]
enum B { Banana = 255, } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
#[repr(C)]
enum C { Chaenomeles = 256, } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
#[repr(C)]
enum P { Peach = 0x1000_0000isize, } // { dg-error "" "" { target *-*-* } }

const TANGERINE: usize = 0x8100_0000; // hack to get negative numbers without negation operator!

#[rustc_layout(debug)]
#[repr(C)]
enum T { Tangerine = TANGERINE as isize } // { dg-error "" "" { target *-*-* } }

