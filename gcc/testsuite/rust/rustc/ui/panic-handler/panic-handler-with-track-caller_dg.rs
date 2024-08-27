//@ compile-flags:-C panic=abort
//@ only-x86_64

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
#[track_caller]
// { dg-error "" "" { target *-*-* } .-1 }
fn panic(info: &PanicInfo) -> ! {
    unimplemented!();
}

