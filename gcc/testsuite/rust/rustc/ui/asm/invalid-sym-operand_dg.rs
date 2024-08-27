//@ needs-asm-support
//@ ignore-nvptx64
//@ ignore-spirv

use std::arch::{asm, global_asm};

// Sym operands must point to a function or static

const C: i32 = 0;
static S: i32 = 0;
global_asm!("{}", sym S);
global_asm!("{}", sym main);
global_asm!("{}", sym C);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    unsafe {
        // Sym operands must point to a function or static

        let x: u64 = 0;
        const C: i32 = 0;
        static S: i32 = 0;
        asm!("{}", sym S);
        asm!("{}", sym main);
        asm!("{}", sym C);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", sym x);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

unsafe fn generic<T>() {
    asm!("{}", sym generic::<T>);
}

