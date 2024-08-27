//@ compile-flags:-C panic=abort

#![feature(lang_items)]
#![feature(no_core)]
#![no_core]
#![no_main]

#[panic_handler]
fn panic() -> ! {
// { dg-error "" "" { target *-*-* } .-1 }
    loop {}
}

#[lang = "sized"]
trait Sized {}

