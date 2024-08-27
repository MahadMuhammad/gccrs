//@ compile-flags:-C panic=abort

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo<'static>) -> !
// { dg-error ".E0308." "" { target *-*-* } .-1 }
{
    loop {}
}

