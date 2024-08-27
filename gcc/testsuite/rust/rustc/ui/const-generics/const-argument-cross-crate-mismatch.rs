//@ aux-build:const_generic_lib.rs

extern crate const_generic_lib;

fn main() {
    let _ = const_generic_lib::function(const_generic_lib::Struct([0u8, 1u8]));
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: const_generic_lib::Alias = const_generic_lib::Struct([0u8, 1u8, 2u8]);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

