//@ only-x86_64

use std::arch::asm;

fn main() {
    let mut foo = 0;
    let mut bar = 0;
    unsafe {
        asm!("", a = in("eax") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{a}", in("eax") foo, a = const bar);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
        asm!("{a}", in("eax") foo, a = const bar);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
        asm!("{1}", in("eax") foo, const bar);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
// { dg-error ".E0435." "" { target *-*-* } .-2 }
    }
}

