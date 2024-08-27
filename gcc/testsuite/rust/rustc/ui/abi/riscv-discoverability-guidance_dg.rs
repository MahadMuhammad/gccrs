// ignore-tidy-linelength
//@ revisions: riscv32 riscv64
//
//@ [riscv32] needs-llvm-components: riscv
//@ [riscv32] compile-flags: --target=riscv32i-unknown-none-elf -C target-feature=-unaligned-scalar-mem --crate-type=rlib
//@ [riscv64] needs-llvm-components: riscv
//@ [riscv64] compile-flags: --target=riscv64gc-unknown-none-elf -C target-feature=-unaligned-scalar-mem --crate-type=rlib
#![no_core]
#![feature(
    no_core,
    lang_items,
    abi_riscv_interrupt
)]
#[lang = "sized"]
trait Sized {}

extern "riscv-interrupt" fn isr() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

extern "riscv-interrupt-u" fn isr_U() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

