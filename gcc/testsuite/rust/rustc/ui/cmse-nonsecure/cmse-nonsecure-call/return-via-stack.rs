//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
#![feature(abi_c_cmse_nonsecure_call, no_core, lang_items)]
#![no_core]
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
impl Copy for u32 {}

#[repr(C)]
pub struct ReprCU64(u64);

#[repr(C)]
pub struct ReprCBytes(u8, u8, u8, u8, u8);

#[repr(C)]
pub struct U64Compound(u32, u32);

#[repr(C, align(16))]
pub struct ReprCAlign16(u16);

#[no_mangle]
pub fn test(
    f1: extern "C-cmse-nonsecure-call" fn() -> ReprCU64, // { dg-error ".E0798." "" { target *-*-* } }
    f2: extern "C-cmse-nonsecure-call" fn() -> ReprCBytes, // { dg-error ".E0798." "" { target *-*-* } }
    f3: extern "C-cmse-nonsecure-call" fn() -> U64Compound, // { dg-error ".E0798." "" { target *-*-* } }
    f4: extern "C-cmse-nonsecure-call" fn() -> ReprCAlign16, // { dg-error ".E0798." "" { target *-*-* } }
    f5: extern "C-cmse-nonsecure-call" fn() -> [u8; 5],  // { dg-error ".E0798." "" { target *-*-* } }
) {
}

#[allow(improper_ctypes_definitions)]
struct Test {
    u128: extern "C-cmse-nonsecure-call" fn() -> u128, // { dg-error ".E0798." "" { target *-*-* } }
    i128: extern "C-cmse-nonsecure-call" fn() -> i128, // { dg-error ".E0798." "" { target *-*-* } }
}

#[repr(C)]
pub union ReprCUnionU64 {
    _unused: u64,
}

#[repr(Rust)]
pub union ReprRustUnionU64 {
    _unused: u64,
}

#[no_mangle]
pub fn test_union(
    f1: extern "C-cmse-nonsecure-call" fn() -> ReprRustUnionU64, // { dg-error ".E0798." "" { target *-*-* } }
    f2: extern "C-cmse-nonsecure-call" fn() -> ReprCUnionU64,    // { dg-error ".E0798." "" { target *-*-* } }
) {
}

