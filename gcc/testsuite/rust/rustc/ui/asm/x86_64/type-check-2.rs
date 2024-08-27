//@ only-x86_64

#![feature(repr_simd, never_type)]

use std::arch::{asm, global_asm};

#[repr(simd)]
struct SimdNonCopy(f32, f32, f32, f32);

fn main() {
    unsafe {
        // Inputs must be initialized

        let x: u64;
        asm!("{}", in(reg) x);
// { dg-error ".E0381." "" { target *-*-* } .-1 }
        let mut y: u64;
        asm!("{}", inout(reg) y);
// { dg-error ".E0381." "" { target *-*-* } .-1 }
        let _ = y;

        // Outputs require mutable places

        let v: Vec<u64> = vec![0, 1, 2];
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        asm!("{}", in(reg) v[0]);
        asm!("{}", out(reg) v[0]);
        asm!("{}", inout(reg) v[0]);

        // Register operands must be Copy

        asm!("{}", in(xmm_reg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
// { dg-error "" "" { target *-*-* } .-1 }

        // Register operands must be integers, floats, SIMD vectors, pointers or
        // function pointers.

        asm!("{}", in(reg) 0i64);
        asm!("{}", in(reg) 0f64);
        asm!("{}", in(xmm_reg) std::arch::x86_64::_mm_setzero_ps());
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

