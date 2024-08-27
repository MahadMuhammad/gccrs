//@ only-aarch64

#![feature(repr_simd, never_type)]

use std::arch::{asm, global_asm};

#[repr(simd)]
#[derive(Clone, Copy)]
struct SimdType(f32, f32, f32, f32);

#[repr(simd)]
struct SimdNonCopy(f32, f32, f32, f32);

fn main() {
    unsafe {
        // Inputs must be initialized

        // Register operands must be Copy

        asm!("{:v}", in(vreg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
// { dg-error "" "" { target *-*-* } .-1 }

        // Register operands must be integers, floats, SIMD vectors, pointers or
        // function pointers.

        asm!("{}", in(reg) 0i64);
        asm!("{}", in(reg) 0f64);
        asm!("{:v}", in(vreg) SimdType(0.0, 0.0, 0.0, 0.0));
        asm!("{}", in(reg) 0 as *const u8);
        asm!("{}", in(reg) 0 as *mut u8);
        asm!("{}", in(reg) main as fn());
        asm!("{}", in(reg) |x: i32| x);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) vec![0]);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) (1, 2, 3));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) [1, 2, 3]);
// { dg-error "" "" { target *-*-* } .-1 }

        // Register inputs (but not outputs) allow references and function types

        let mut f = main;
        let mut r = &mut 0;
        asm!("{}", in(reg) f);
        asm!("{}", inout(reg) f);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) r);
        asm!("{}", inout(reg) r);
// { dg-error "" "" { target *-*-* } .-1 }
        let _ = (f, r);

        // Type checks ignore never type

        let u: ! = unreachable!();
        asm!("{}", in(reg) u);
    }
}

