// { dg-additional-options "-frust-edition=2018" }
//@ aux-build:builtin-attrs.rs
#![feature(decl_macro)] // { dg-error ".E0659." "" { target *-*-* } }

extern crate builtin_attrs;
use builtin_attrs::*;
use builtin_attrs::{bench, test};

#[repr(C)] // { dg-error ".E0659." "" { target *-*-* } }
struct S;
#[cfg_attr(all(), repr(C))] // { dg-error ".E0659." "" { target *-*-* } }
struct SCond;

#[test] // OK, shadowed
fn test() {}

#[bench] // OK, shadowed
fn bench() {}

fn non_macro_expanded_location<#[repr(C)] T>() {
// { dg-error ".E0517." "" { target *-*-* } .-1 }
// { dg-error ".E0517." "" { target *-*-* } .-2 }
    match 0u8 {
        #[repr(C)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
// { dg-error ".E0517." "" { target *-*-* } .-2 }
        _ => {}
    }
}

fn main() {
    Test;
    Bench;
    NonExistent; // { dg-error ".E0425." "" { target *-*-* } }
}

use deny as allow;
#[allow(unused)] // { dg-error ".E0659." "" { target *-*-* } }
fn builtin_renamed() {}

