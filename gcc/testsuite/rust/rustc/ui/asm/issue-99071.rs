//@ compile-flags: --target thumbv6m-none-eabi
//@ needs-llvm-components: arm
//@ needs-asm-support

#![feature(no_core, lang_items, rustc_attrs)]
#![no_core]
#![crate_type = "rlib"]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}
#[lang = "sized"]
trait Sized {}

pub fn foo() {
    unsafe {
        asm!("", in("r8") 0);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

