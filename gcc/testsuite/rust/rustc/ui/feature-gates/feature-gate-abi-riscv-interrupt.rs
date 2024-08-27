//@ needs-llvm-components: riscv
//@ compile-flags: --target=riscv32imc-unknown-none-elf --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang = "sized"]
trait Sized {}

// Test that the riscv interrupt ABIs cannot be used when riscv_interrupt
// feature gate is not used.

extern "riscv-interrupt-m" fn f() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
extern "riscv-interrupt-s" fn f_s() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }

trait T {
    extern "riscv-interrupt-m" fn m();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

struct S;
impl T for S {
    extern "riscv-interrupt-m" fn m() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

impl S {
    extern "riscv-interrupt-m" fn im() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

type TA = extern "riscv-interrupt-m" fn();
// { dg-error ".E0658." "" { target *-*-* } .-1 }

