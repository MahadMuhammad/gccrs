//@ only-aarch64
//@ compile-flags: -C target-feature=+neon

use std::arch::asm;

fn main() {
    let mut foo = 0;
    let mut bar = 0;
    unsafe {
        // Bad register/register class

        asm!("{}", in(foo) foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("foo") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{:z}", in(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{:r}", in(vreg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{:r}", in(vreg_low16) foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{:a}", const 0);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{:a}", sym main);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("x29") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("sp") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("xzr") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("x19") foo);
// { dg-error "" "" { target *-*-* } .-1 }

        asm!("", in("p0") foo);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("", out("p0") _);
        asm!("{}", in(preg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("{}", out(preg) _);
// { dg-error "" "" { target *-*-* } .-1 }

        // Explicit register conflicts
        // (except in/lateout which don't conflict)

        asm!("", in("x0") foo, in("w0") bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("x0") foo, out("x0") bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("w0") foo, lateout("w0") bar);
        asm!("", in("v0") foo, in("q0") bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("v0") foo, out("q0") bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", in("v0") foo, lateout("q0") bar);
    }
}

