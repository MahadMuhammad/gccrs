//@ needs-asm-support
//@ ignore-nvptx64
//@ ignore-spirv

use std::arch::{asm, global_asm};

// Const operands must be integers and must be constants.

global_asm!("{}", const 0);
global_asm!("{}", const 0i32);
global_asm!("{}", const 0i128);
global_asm!("{}", const 0f32);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", const 0 as *mut u8);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    unsafe {
        // Const operands must be integers and must be constants.

        asm!("{}", const 0);
        asm!("{}", const 0i32);
        asm!("{}", const 0i128);
        asm!("{}", const 0f32);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", const 0 as *mut u8);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", const &0);
// { dg-error "" "" { target *-*-* } .-1 }

        // Constants must be... constant

        let x = 0;
        const fn const_foo(x: i32) -> i32 {
            x
        }
        const fn const_bar<T>(x: T) -> T {
            x
        }
        asm!("{}", const x);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
        asm!("{}", const const_foo(0));
        asm!("{}", const const_foo(x));
// { dg-error ".E0435." "" { target *-*-* } .-1 }
        asm!("{}", const const_bar(0));
        asm!("{}", const const_bar(x));
// { dg-error ".E0435." "" { target *-*-* } .-1 }
    }
}

