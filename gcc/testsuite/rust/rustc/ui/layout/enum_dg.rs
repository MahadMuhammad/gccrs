//@ normalize-stderr-test: "pref: Align\([1-8] bytes\)" -> "pref: $$PREF_ALIGN"
//! Various enum layout tests.

#![feature(rustc_attrs)]
#![feature(never_type)]
#![crate_type = "lib"]

#[rustc_layout(align)]
enum UninhabitedVariantAlign { // { dg-error "" "" { target *-*-* } }
    A([u8; 32]),
    B([u16; 0], !), // make sure alignment in uninhabited fields is respected
}

#[rustc_layout(size)]
enum UninhabitedVariantSpace { // { dg-error "" "" { target *-*-* } }
    A,
    B([u8; 15], !), // make sure there is space being reserved for this field.
}

#[rustc_layout(abi)]
enum ScalarPairDifferingSign { // { dg-error "" "" { target *-*-* } }
    A(u8),
    B(i8),
}

