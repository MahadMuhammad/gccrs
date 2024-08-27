//@ revisions: x86_64 arm arm_llvm_18
//@[x86_64] compile-flags: --target x86_64-unknown-linux-gnu
//@[x86_64] check-pass
//@[x86_64] needs-llvm-components: x86
//@[arm] compile-flags: --target armv7-unknown-linux-gnueabihf
//@[arm] build-fail
//@[arm] needs-llvm-components: arm
//@[arm] ignore-llvm-version: 18 - 99
//Newer LLVM produces extra error notes.
//@[arm_llvm_18] compile-flags: --target armv7-unknown-linux-gnueabihf
//@[arm_llvm_18] build-fail
//@[arm_llvm_18] needs-llvm-components: arm
//@[arm_llvm_18] min-llvm-version: 18
//@ needs-asm-support

#![feature(no_core, lang_items, rustc_attrs)]
#![crate_type = "rlib"]
#![no_core]


#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! global_asm {
    () => {};
}

#[lang = "sized"]
trait Sized {}

pub fn main() {
    unsafe {
        asm!(".intel_syntax noprefix", "nop");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        asm!(".intel_syntax aaa noprefix", "nop");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        asm!(".att_syntax noprefix", "nop");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        asm!(".att_syntax bbb noprefix", "nop");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        asm!(".intel_syntax noprefix; nop");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(
            r"
            .intel_syntax noprefix
            nop"
        );
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
    }
}

global_asm!(".intel_syntax noprefix", "nop");
// { dg-warning "" "" { target *-*-* } .-1 }
// Assembler errors don't have line numbers, so no error on ARM

