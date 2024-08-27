// Tests for repeating attribute warnings.
//@ aux-build:lint_unused_extern_crate.rs
//@ compile-flags:--test
// Not tested due to extra requirements:
// - panic_handler: needs extra setup
// - target_feature: platform-specific
// - link_section: platform-specific
// - proc_macro, proc_macro_derive, proc_macro_attribute: needs to be a
//   proc-macro, and have special handling for mixing.
// - unstable attributes (not going to bother)
// - no_main: extra setup
#![deny(unused_attributes)]
#![crate_name = "unused_attr_duplicate"]
#![crate_name = "unused_attr_duplicate2"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
#![recursion_limit = "128"]
#![recursion_limit = "256"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
#![type_length_limit = "1048576"]
#![type_length_limit = "1"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
#![no_std]
#![no_std] // { dg-error "" "" { target *-*-* } }
#![no_implicit_prelude]
#![no_implicit_prelude] // { dg-error "" "" { target *-*-* } }
#![windows_subsystem = "console"]
#![windows_subsystem = "windows"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
#![no_builtins]
#![no_builtins] // { dg-error "" "" { target *-*-* } }

#[no_link]
#[no_link] // { dg-error "" "" { target *-*-* } }
extern crate lint_unused_extern_crate;

#[macro_use]
#[macro_use] // { dg-error "" "" { target *-*-* } }
pub mod m {
    #[macro_export]
    #[macro_export] // { dg-error "" "" { target *-*-* } }
    macro_rules! foo {
        () => {};
    }
}

#[path = "auxiliary/lint_unused_extern_crate.rs"]
#[path = "bar.rs"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
pub mod from_path;

#[test]
#[ignore]
#[ignore = "some text"] // { dg-error "" "" { target *-*-* } }
#[should_panic]
#[should_panic(expected = "values don't match")] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
fn t1() {}

#[must_use]
#[must_use = "some message"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
// No warnings for #[repr], would require more logic.
#[repr(C)]
#[repr(C)]
#[non_exhaustive]
#[non_exhaustive] // { dg-error "" "" { target *-*-* } }
pub struct X;

#[automatically_derived]
#[automatically_derived] // { dg-error "" "" { target *-*-* } }
impl X {}

#[inline(always)]
#[inline(never)] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
#[cold]
#[cold] // { dg-error "" "" { target *-*-* } }
#[track_caller]
#[track_caller] // { dg-error "" "" { target *-*-* } }
pub fn xyz() {}

// No warnings for #[link], would require more logic.
#[link(name = "rust_test_helpers", kind = "static")]
#[link(name = "rust_test_helpers", kind = "static")]
extern "C" {
    #[link_name = "this_does_not_exist"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
    #[link_name = "rust_dbg_extern_identity_u32"]
    pub fn name_in_rust(v: u32) -> u32;
}

#[export_name = "exported_symbol_name"] // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
#[export_name = "exported_symbol_name2"]
pub fn export_test() {}

#[no_mangle]
#[no_mangle] // { dg-error "" "" { target *-*-* } }
pub fn no_mangle_test() {}

#[used]
#[used] // { dg-error "" "" { target *-*-* } }
static FOO: u32 = 0;

fn main() {}

