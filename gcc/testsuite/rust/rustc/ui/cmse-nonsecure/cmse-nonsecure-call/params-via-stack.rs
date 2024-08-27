//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
#![feature(abi_c_cmse_nonsecure_call, no_core, lang_items)]
#![no_core]
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
impl Copy for u32 {}

#[repr(C, align(16))]
#[allow(unused)]
pub struct AlignRelevant(u32);

#[no_mangle]
pub fn test(
    f1: extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32, x: u32, y: u32), // { dg-error ".E0798." "" { target *-*-* } }
    f2: extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u16, u16),            // { dg-error ".E0798." "" { target *-*-* } }
    f3: extern "C-cmse-nonsecure-call" fn(u32, u64, u32),                      // { dg-error ".E0798." "" { target *-*-* } }
    f4: extern "C-cmse-nonsecure-call" fn(AlignRelevant, u32),                 // { dg-error ".E0798." "" { target *-*-* } }
    f5: extern "C-cmse-nonsecure-call" fn([u32; 5]),                           // { dg-error ".E0798." "" { target *-*-* } }
) {
}

