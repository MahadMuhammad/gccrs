//@ needs-asm-support
//@ compile-flags: --test

#![allow(undefined_naked_function_abi)]
#![feature(naked_functions)]
#![feature(test)]
#![crate_type = "lib"]

use std::arch::asm;

#[test]
#[naked]
// { dg-error ".E0736." "" { target *-*-* } .-1 }
fn test_naked() {
    unsafe { asm!("", options(noreturn)) };
}

#[should_panic]
#[test]
#[naked]
// { dg-error ".E0736." "" { target *-*-* } .-1 }
fn test_naked_should_panic() {
    unsafe { asm!("", options(noreturn)) };
}

#[ignore]
#[test]
#[naked]
// { dg-error ".E0736." "" { target *-*-* } .-1 }
fn test_naked_ignore() {
    unsafe { asm!("", options(noreturn)) };
}

#[bench]
#[naked]
// { dg-error ".E0736." "" { target *-*-* } .-1 }
fn bench_naked() {
    unsafe { asm!("", options(noreturn)) };
}

