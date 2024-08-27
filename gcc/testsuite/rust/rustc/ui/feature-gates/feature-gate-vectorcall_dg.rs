// gate-test-abi_vectorcall
//@ needs-llvm-components: x86
//@ compile-flags: --target=i686-pc-windows-msvc --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang="sized"]
trait Sized { }

// Test that the "vectorcall" ABI is feature-gated, and cannot be used when
// the `vectorcall` feature gate is not used.

extern "vectorcall" fn f() {} // { dg-error ".E0658." "" { target *-*-* } }

trait T {
    extern "vectorcall" fn m(); // { dg-error ".E0658." "" { target *-*-* } }

    extern "vectorcall" fn dm() {} // { dg-error ".E0658." "" { target *-*-* } }
}

struct S;
impl T for S {
    extern "vectorcall" fn m() {} // { dg-error ".E0658." "" { target *-*-* } }
}

impl S {
    extern "vectorcall" fn im() {} // { dg-error ".E0658." "" { target *-*-* } }
}

type TA = extern "vectorcall" fn(); // { dg-error ".E0658." "" { target *-*-* } }

extern "vectorcall" {} // { dg-error ".E0658." "" { target *-*-* } }

