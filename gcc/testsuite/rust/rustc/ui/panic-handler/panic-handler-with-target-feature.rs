//@ compile-flags:-C panic=abort
//@ only-x86_64

#![feature(target_feature_11)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
#[target_feature(enable = "avx2")]
// { dg-error "" "" { target *-*-* } .-1 }
fn panic(info: &PanicInfo) -> ! {
    unimplemented!();
}

