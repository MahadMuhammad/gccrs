//@ needs-asm-support
//@ only-x86_64

use std::arch::asm;

// checks various modes of failure for the `clobber_abi` argument (after parsing)

fn main() {
    unsafe {
        asm!("", clobber_abi("C"));
        asm!("", clobber_abi("foo"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("C", "foo"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("C", "C"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("win64", "sysv64"));
        asm!("", clobber_abi("win64", "efiapi"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("C", "foo", "C"));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("", clobber_abi("win64", "foo", "efiapi"));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("", clobber_abi("C"), clobber_abi("C"));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("win64"), clobber_abi("sysv64"));
        asm!("", clobber_abi("win64"), clobber_abi("efiapi"));
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

