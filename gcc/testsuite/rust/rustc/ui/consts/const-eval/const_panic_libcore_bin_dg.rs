#![crate_type = "bin"]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

const Z: () = panic!("cheese");
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const Y: () = unreachable!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

const X: () = unimplemented!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

#[lang = "eh_personality"]
fn eh() {}
#[lang = "eh_catch_typeinfo"]
static EH_CATCH_TYPEINFO: u8 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

