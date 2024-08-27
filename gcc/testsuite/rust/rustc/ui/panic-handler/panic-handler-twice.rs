//@ dont-check-compiler-stderr
//@ aux-build:some-panic-impl.rs

#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate some_panic_impl;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
// { dg-error "" "" { target *-*-* } .-1 }
    loop {}
}

#[lang = "eh_personality"]
fn eh() {}

