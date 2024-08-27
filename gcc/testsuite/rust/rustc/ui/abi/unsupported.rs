//@ revisions: x64 i686 aarch64 arm riscv32 riscv64
//
//@ [x64] needs-llvm-components: x86
//@ [x64] compile-flags: --target=x86_64-unknown-linux-gnu --crate-type=rlib
//@ [i686] needs-llvm-components: x86
//@ [i686] compile-flags: --target=i686-unknown-linux-gnu --crate-type=rlib
//@ [aarch64] needs-llvm-components: aarch64
//@ [aarch64] compile-flags: --target=aarch64-unknown-linux-gnu --crate-type=rlib
//@ [arm] needs-llvm-components: arm
//@ [arm] compile-flags: --target=armv7-unknown-linux-gnueabihf --crate-type=rlib
//@ [riscv32] needs-llvm-components: riscv
//@ [riscv32] compile-flags: --target=riscv32i-unknown-none-elf --crate-type=rlib
//@ [riscv64] needs-llvm-components: riscv
//@ [riscv64] compile-flags: --target=riscv64gc-unknown-none-elf --crate-type=rlib
#![no_core]
#![feature(
    no_core,
    lang_items,
    abi_ptx,
    abi_msp430_interrupt,
    abi_avr_interrupt,
    abi_x86_interrupt,
    abi_riscv_interrupt
)]
#[lang = "sized"]
trait Sized {}

extern "ptx-kernel" fn ptx() {}
// { dg-error "" "" { target *-*-* } .-1 }
extern "aapcs" fn aapcs() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
extern "msp430-interrupt" fn msp430() {}
// { dg-error "" "" { target *-*-* } .-1 }
extern "avr-interrupt" fn avr() {}
// { dg-error "" "" { target *-*-* } .-1 }
extern "riscv-interrupt-m" fn riscv() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
extern "x86-interrupt" fn x86() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
extern "thiscall" fn thiscall() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
extern "stdcall" fn stdcall() {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
// { dg-warning "" "" { target *-*-* } .-7 }
// { dg-warning "" "" { target *-*-* } .-8 }
// { dg-warning "" "" { target *-*-* } .-9 }
// { dg-warning "" "" { target *-*-* } .-10 }

