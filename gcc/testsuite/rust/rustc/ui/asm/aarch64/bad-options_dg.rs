//@ only-aarch64

use std::arch::{asm, global_asm};

fn main() {
    let mut foo = 0;
    unsafe {
        asm!("", options(nomem, readonly));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(pure, nomem, noreturn));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("{}", in(reg) foo, options(pure, nomem));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", out(reg) foo, options(noreturn));
// { dg-error "" "" { target *-*-* } .-1 }
    }

    unsafe {
        asm!("", clobber_abi("foo"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", out(reg) foo, clobber_abi("C"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", out("x0") foo, clobber_abi("C"));
    }
}

global_asm!("", options(nomem));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(readonly));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(noreturn));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(pure));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(nostack));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(preserves_flags));
// { dg-error "" "" { target *-*-* } .-1 }

