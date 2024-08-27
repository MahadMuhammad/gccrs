//@ only-aarch64
//@ needs-asm-support
//@ run-rustfix

use std::arch::asm;

fn main() {
    unsafe {
        asm!("", options(nomem, nomem));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(preserves_flags, preserves_flags));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(nostack, preserves_flags), options(nostack));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(nostack, nostack), options(nostack), options(nostack));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        asm!(
            "",
            options(nomem, noreturn),
            options(preserves_flags, noreturn), // { dg-error "" "" { target *-*-* } }
            options(nomem, nostack),            // { dg-error "" "" { target *-*-* } }
            options(noreturn),                  // { dg-error "" "" { target *-*-* } }
        );
    }
}

